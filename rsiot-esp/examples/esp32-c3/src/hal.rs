//! Взаимодействие с HAL ESP32

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{
        gpio::{Input, InputPin, Level, PinDriver},
        peripherals::Peripherals,
    },
    wifi::{AccessPointConfiguration, BlockingWifi, Configuration, EspWifi},
};
use rgb::RGB8;
use rsiot::message::{msg_types::Value, IMessage};
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
    set.spawn(gpio_read(
        input.resubscribe(),
        output.clone(),
        button,
        |level| Message::Button(Value::new(*level)),
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

async fn gpio_read<'a, TPin, TMessage>(
    _input: broadcast::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    mut driver: PinDriver<'a, TPin, Input>,
    fn_output: fn(&bool) -> TMessage,
) where
    TPin: InputPin,
{
    loop {
        driver.wait_for_any_edge().await.unwrap();
        let level = driver.get_level();
        let level = gpio_level_to_bool(&level);
        let msg = (fn_output)(&level);
        output.send(msg).await.unwrap();
    }
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

fn gpio_level_to_bool(level: &Level) -> bool {
    match level {
        Level::Low => true,
        Level::High => false,
    }
}

fn temperature(val: f32) -> String {
    templated(format!("Chip temperature: {:.2}°C", val))
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}
