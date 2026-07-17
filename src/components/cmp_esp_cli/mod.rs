//! CLI для микроконтроллера ESP32.

mod error;

const COMPONENT_NAME: &str = "cmp_esp_cli";

use std::{
    fmt::Debug,
    io::{Read, Write},
    time::Duration,
};

use clap::Parser;
use esp_idf_svc::{
    hal::{
        gpio::{AnyIOPin, PinDriver, Pull},
        usb_serial::{USB_SERIAL, UsbDMinGpio, UsbDPlusGpio, UsbSerialDriver},
    },
    io::vfs::BlockingStdIo,
};
use tokio::task::JoinSet;
use tracing::info;

use error::Error;

#[derive(Debug)]
enum WaitMsg {
    Timeout,
    Symbol,
}

/// Результат обработки команды CLI
pub enum CliProcessResult {
    /// Продолжить выполнение
    Continue,

    /// Сохранить конфигурацию и выйти
    SaveAndExit,

    /// Выйти без сохранения конфигурации
    Exit,
}

/// Настройка и запуск CLI
pub struct EspCli<TCLi, TConfig>
where
    TCLi: Debug + Parser,
    TConfig: Clone,
{
    /// Ссылка на аппаратный интерфейс I2C
    pub usb_serial: USB_SERIAL<'static>,

    /// Пин USB D-
    pub pin_usb_d_min: UsbDMinGpio<'static>,

    /// Пин USB D+
    pub pin_usb_d_plus: UsbDPlusGpio<'static>,

    /// Пин для запуска CLI. Используется кнопка BOOT
    pub pin_start_cli: AnyIOPin<'static>,

    /// Настройки, сохранённые в микроконтроллере перед запуском CLI
    pub saved_config: TConfig,

    /// Функция для обработки команд CLI
    pub fn_process: fn(TCLi, &mut TConfig) -> CliProcessResult,
}
impl<TCLi, TConfig> EspCli<TCLi, TConfig>
where
    TCLi: Debug + Parser,
    TConfig: Clone,
{
    /// Запустить выполнение
    pub async fn run(self) -> Result<TConfig, Error> {
        let driver = UsbSerialDriver::new(
            self.usb_serial,
            self.pin_usb_d_min,
            self.pin_usb_d_plus,
            &esp_idf_svc::hal::usb_serial::config::Config::default(),
        )
        .map_err(Error::CreateUsbSerialDriver)?;

        // Если USB кабель не подключен, то выходим из функции и запускаем программу
        if !driver.is_connected() {
            info!("USB not connected, starting program");
            return Ok(self.saved_config);
        }

        // Ждём нажатия кнопки BOOT
        info!("Press the BOOT button to change the settings");
        let mut pin_start_cli =
            PinDriver::input(self.pin_start_cli, Pull::Up).map_err(Error::PinDriver)?;

        let mut join_set: JoinSet<Result<WaitMsg, Error>> = JoinSet::new();

        join_set.spawn(async move {
            // Ожидание в секундах
            let mut count = 5;

            while count > 0 {
                println!("Нажмите BOOT для изменения настроек: {}", count);
                tokio::time::sleep(Duration::from_millis(1_000)).await;
                count -= 1;
            }

            Ok(WaitMsg::Timeout)
        });

        join_set.spawn(async move {
            pin_start_cli
                .wait_for_falling_edge()
                .await
                .map_err(Error::GpioWaitForEdge)?;

            Ok(WaitMsg::Symbol)
        });

        let Some(result) = join_set.join_next().await else {
            return Ok(self.saved_config);
        };
        join_set.shutdown().await;
        let msg = result.map_err(Error::TokioTaskJoin)??;
        match msg {
            WaitMsg::Timeout => {
                info!("BOOT button not pressed, starting program");
                return Ok(self.saved_config);
            }
            WaitMsg::Symbol => info!("Запуск командного интерфейса настроек"),
        };

        // Нажали BOOT, запускаем CLI
        let _blocking_io = BlockingStdIo::usb_serial(driver).map_err(Error::BlockingStdIo)?;

        let mut new_config = self.saved_config.clone();

        loop {
            let input_string = read_line()?;

            let args = TCLi::try_parse_from(input_string.split(" "));

            match args {
                Ok(v) => {
                    let parse_result = (self.fn_process)(v, &mut new_config);

                    match parse_result {
                        CliProcessResult::Continue => {}
                        CliProcessResult::SaveAndExit => {
                            return Ok(new_config);
                        }
                        CliProcessResult::Exit => return Ok(self.saved_config),
                    }
                }
                Err(err) => println!("{}", err),
            }
        }
    }
}

fn read_line() -> Result<String, Error> {
    const BACKSPACE: char = 8u8 as char;
    const RETURN: u8 = 0x0A;

    println!("--------------------------------------------------------------------------------");
    println!("Введите команду: ");

    let mut input_string = String::new();
    loop {
        let mut buffer = [0; 10];

        let read_size = std::io::stdin()
            .read(&mut buffer)
            .map_err(Error::StdinRead)?;

        if buffer[0] == 8u8 {
            input_string.pop();
            print!("{} {}", BACKSPACE, BACKSPACE);
        } else if buffer[0] == RETURN {
            print!("{}", RETURN as char);
            break;
        } else {
            let new_char = String::from_utf8_lossy(&buffer[..read_size]);
            input_string.push_str(&new_char);
            print!("{}", new_char)
        }
        std::io::stdout().flush().map_err(Error::StdoutFlush)?;
    }
    Ok(input_string)
}
