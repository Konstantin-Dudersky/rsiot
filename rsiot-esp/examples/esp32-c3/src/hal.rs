//! Взаимодействие с HAL ESP32

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{gpio::PinDriver, peripherals::Peripherals},
    wifi::{AccessPointConfiguration, AuthMethod, ClientConfiguration, Configuration, EspWifi},
};
use rgb::RGB8;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use rsiot::message::{msg_types::Value, IMessage};
use rsiot_esp::hardware_tasks::{
    gpio_input, gpio_output, wifi_setup_access_point, GpioOutputConfig,
};

use super::message::Message;
use super::ws2812rmt::WS2812RMT;

pub struct Config;

pub async fn hal(
    input: broadcast::Receiver<Message>,
    output: mpsc::Sender<Message>,
    _config: Config,
) {
    let mut set: JoinSet<()> = JoinSet::new();

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    // let nvs = EspDefaultNvsPartition::take().unwrap();

    // включаем реле на gpio2
    let relay = PinDriver::output(peripherals.pins.gpio2).unwrap();
    set.spawn(gpio_output(
        input.resubscribe(),
        output.clone(),
        GpioOutputConfig {
            driver: relay,
            fn_input: |msg| match msg {
                Message::Relay2(val) => Some(val.value),
                _ => None,
            },
            is_low_triggered: false,
        },
    ));

    // отправляем код цвета на LED gpio8
    let led = WS2812RMT::new(peripherals.pins.gpio8, peripherals.rmt.channel0).unwrap();
    set.spawn(ws2812(
        input.resubscribe(),
        output.clone(),
        led,
        |msg| match msg {
            Message::SetLedColor(val) => Some(val.value),
            _ => None,
        },
    ));

    // читаем кнопку с gpio9
    let button = PinDriver::input(peripherals.pins.gpio9).unwrap();
    set.spawn(gpio_input(
        input.resubscribe(),
        output.clone(),
        button,
        |level| Message::Button(Value::new(*level)),
    ));

    // настраиваем Wi-Fi
    let mut wifi = EspWifi::new(peripherals.modem, sys_loop.clone(), None).unwrap();
    wifi_setup_access_point(
        &mut wifi,
        sys_loop.clone(),
        Configuration::Mixed(
            ClientConfiguration {
                ssid: "Fermenter".into(),
                password: "k33n3+Ik".into(),
                auth_method: AuthMethod::None,
                ..Default::default()
            },
            AccessPointConfiguration {
                ssid: "test_esp_ap".into(),
                ..Default::default()
            },
        ),
    );

    while (set.join_next().await).is_some() {}
}

async fn ws2812<'a, TMessage>(
    mut input: broadcast::Receiver<TMessage>,
    _output: mpsc::Sender<TMessage>,
    mut driver: WS2812RMT<'a>,
    fn_input: fn(&TMessage) -> Option<RGB8>,
) where
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
        let color = (fn_input)(&msg);
        let color = match color {
            Some(val) => val,
            None => continue,
        };
        driver.set_pixel(color).unwrap();
    }
}
