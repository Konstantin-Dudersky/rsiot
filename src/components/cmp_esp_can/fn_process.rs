use std::{thread, time::Duration};

use enumset::EnumSet;
use esp_idf_svc::{
    hal::{
        can::{self, Alert, CanDriver},
        delay::TickType,
    },
    sys::{ESP_ERR_TIMEOUT, esp, twai_initiate_recovery},
};
use tokio::{
    sync::mpsc::{Receiver, Sender, error::TryRecvError},
    task::{JoinHandle, JoinSet},
};
use tracing::{info, warn};

use crate::{
    components::shared_tasks::cmp_can_general::CanGeneralTasks,
    components_config::can_general::CanFrame, executor::MsgBusLinker, message::MsgDataBound,
};

use super::{Config, Error, can_filter::can_filter_convert};

pub async fn fn_process<TMsg, TFnInput>(
    config: Config<TMsg, TFnInput>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
    TFnInput: 'static + Fn(&TMsg) -> anyhow::Result<Option<Vec<CanFrame>>> + Send + Sync,
{
    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    // Общие задачи обмена по шине CAN
    let (mut ch_rx_send_to_can, ch_tx_recv_from_can) = CanGeneralTasks {
        msgbus_linker,
        task_set: &mut task_set,
        fn_input: config.fn_input,
        fn_output: config.fn_output,
        error_task_end_input: || Error::TaskEndInput,
        error_task_end_output: || Error::TaskEndOutput,
        error_tokio_mpsc_send: || Error::TokioSyncMpscSend,
    }
    .spawn();

    // Фильтр
    let filter = can_filter_convert(&config.filters)?;

    // Скорость
    let timing = config.can_settings.bitrate.into();

    // Выбираем ошибки, которые драйвер будет выдавать при работе
    //
    // Из полного перечня исключены:
    // - Alert::AboveErrorWarning
    // - Alert::BelowErrorWarning
    // - Alert::PeripheralReset - создаётся очень много сообщений
    // - Alert::Success - возникает при успешной отправке - нет необходимости
    // - Alert::TransmitIdle - возникает после отправки - нет необходимости
    //
    // Описание ошибок в документации:
    // https://docs.espressif.com/projects/esp-idf/en/v4.2/esp32/api-reference/peripherals/twai.html
    let mut alerts = EnumSet::new();
    alerts.insert(Alert::ActiveError); // Состояние ERROR ACTIVE
    alerts.insert(Alert::AlertAndLog);
    alerts.insert(Alert::ArbLost);
    alerts.insert(Alert::BusError);
    alerts.insert(Alert::BusOffline); // Состояние BUS OFF
    alerts.insert(Alert::BusRecovered);
    alerts.insert(Alert::ErrorPass); // Состояние ERROR PASSIVE
    alerts.insert(Alert::ReceiveFifoOverflow);
    alerts.insert(Alert::ReceiveQueueFull);
    alerts.insert(Alert::Received);
    alerts.insert(Alert::RecoveryInProgress);
    alerts.insert(Alert::TransmitFailed);
    alerts.insert(Alert::TransmitRetried);

    // Настройка драйвера CAN
    let can_driver_config = can::config::Config::new()
        .filter(filter)
        .timing(timing)
        .rx_queue_len(100)
        .tx_queue_len(100)
        .alerts(alerts);

    // Асинхронный драйвер часто падает. Поэтому используется синхронная в отдельном потоке
    let handle: JoinHandle<Result<(), Error>> = tokio::task::spawn_blocking(move || {
        let mut can_driver =
            CanDriver::new(config.can, config.pin_tx, config.pin_rx, &can_driver_config)
                .map_err(Error::DriverCreate)?;

        can_driver_start(&mut can_driver)?;

        loop {
            // Отправка кадров
            step_transmit(
                &mut can_driver,
                &mut ch_rx_send_to_can,
                Duration::from_millis(10),
            )?;

            // Получение кадров
            step_receive(
                &mut can_driver,
                &ch_tx_recv_from_can,
                Duration::from_millis(10),
            )?;

            // Проверка ошибок
            step_read_alerts(&mut can_driver, Duration::from_millis(1))?;

            thread::sleep(Duration::from_millis(50));
        }
    });

    handle.await??;

    Ok(())
}

/// Шаг получения кадров
fn step_receive<'a>(
    can_driver: &mut CanDriver<'a>,
    ch_tx_recv_from_can: &Sender<CanFrame>,
    timeout: Duration,
) -> Result<(), Error> {
    // Ожидаем получение кадра с таймаутом
    let result = can_driver.receive(dur_to_ticks(timeout));

    let frame = match result {
        Ok(frame) => frame,
        Err(e) => {
            if e.code() != ESP_ERR_TIMEOUT {
                warn!("Error receiving CAN frame: {:?}", e);
            }
            return Ok(());
        }
    };

    // Конвертируем данные
    let frame: CanFrame = frame.into();

    // Отправка полученного кадра для дальнейшей обработки
    let res = ch_tx_recv_from_can.try_send(frame);
    if let Err(err) = res {
        warn!("Output channel full: {err}");
    }

    Ok(())
}

/// Шаг отправки кадров
fn step_transmit<'a>(
    can_driver: &mut CanDriver<'a>,
    ch_rx_send_to_can: &mut Receiver<CanFrame>,
    timeout: Duration,
) -> Result<(), Error> {
    // Проверяем наличие кадров для отправки
    let frame = ch_rx_send_to_can.try_recv();
    let frame = match frame {
        Ok(val) => val,
        Err(err) => match err {
            TryRecvError::Empty => return Ok(()),
            TryRecvError::Disconnected => todo!(),
        },
    };

    // Конвертируем данные
    let frame_conv: Result<esp_idf_svc::hal::can::Frame, _> = frame.try_into();
    let frame = match frame_conv {
        Ok(v) => v,
        Err(e) => {
            warn!("Error converting CAN frame: {:?}", e);
            return Ok(());
        }
    };

    // Отправка кадра
    let res = can_driver.transmit(&frame, dur_to_ticks(timeout));
    if let Err(err) = res {
        warn!("Error transmitting CAN frame: {:?}", err);
    }
    Ok(())
}

/// Шаг проверки сообщений и перезагрузка адаптера
fn step_read_alerts<'a>(can_driver: &mut CanDriver<'a>, timeout: Duration) -> Result<(), Error> {
    let result = can_driver.read_alerts(dur_to_ticks(timeout));
    match result {
        Ok(alerts) => {
            warn!("CAN alerts: {:?}", alerts);
            // if alerts.contains(Alert::ErrorPass) {
            //     can_driver_recovery()?;
            // }
            // if alerts.contains(Alert::RecoveryInProgress) {
            //     can_driver_start(can_driver)?;
            // }
        }
        Err(e) => {
            if e.code() != ESP_ERR_TIMEOUT {
                warn!("Error receiving CAN alerts: {:?}", e);
            }
        }
    };
    Ok(())
}

/// Запуск драйвера CAN
fn can_driver_start<'a>(can_driver: &mut CanDriver<'a>) -> Result<(), Error> {
    info!("CAN driver - start");
    let res = can_driver.start();
    if let Err(err) = res {
        warn!("Error starting CAN driver: {:?}", err);
        return Err(Error::DriverStart(err));
    }
    Ok(())
}

/// Восстановление драйвера CAN
///
/// Используется unsafe, поскольку в API AsyncCanDriver нет методов
fn can_driver_recovery() -> Result<(), Error> {
    info!("CAN driver - recovery");
    let res = esp!(unsafe { twai_initiate_recovery() });
    if let Err(err) = res {
        warn!("Error recovery CAN driver: {:?}", err);
        return Err(Error::DriverRecovery(err));
    }
    Ok(())
}

fn dur_to_ticks(dur: Duration) -> u32 {
    let millis = dur.as_millis() as u64;
    let tick = TickType::new_millis(millis);
    tick.ticks()
}
