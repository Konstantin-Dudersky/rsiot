//! Взаимодействие с HAL ESP32

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{gpio::PinDriver, peripherals::Peripherals},
    wifi::{AccessPointConfiguration, BlockingWifi, Configuration, EspWifi},
};
use rgb::RGB8;
use rsiot::message::{msg_types::Value, IMessage};
use rsiot_esp::hardware_tasks::{gpio_input, gpio_output, GpioOutputConfig};
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};
use tracing::info;

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

    // читаем кнопку с gpio9
    let button = PinDriver::input(peripherals.pins.gpio9).unwrap();
    set.spawn(gpio_input(
        input.resubscribe(),
        output.clone(),
        button,
        |level| Message::Button(Value::new(*level)),
    ));

    // включаем реле
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

    // отправляем код цвета на LED
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

    // настраиваем Wi-Fi
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), None).unwrap(),
        sys_loop.clone(),
    )
    .unwrap();
    wifi_acces_point(&mut wifi);

    while (set.join_next().await).is_some() {}
}

fn wifi_acces_point<'a>(wifi: &mut BlockingWifi<EspWifi<'a>>) {
    let wifi_configuration: Configuration = Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "test_esp_ap".into(),
        ..Default::default()
    });
    wifi.set_configuration(&wifi_configuration).unwrap();
    wifi.start().unwrap();
    info!("is wifi started: {:?}", wifi.is_started());
    info!("{:?}", wifi.get_capabilities());
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
