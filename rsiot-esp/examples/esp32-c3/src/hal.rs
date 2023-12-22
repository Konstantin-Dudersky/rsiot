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
use rsiot::{
    component::{cmp_mpsc_to_mpsc, cmpbase_mpsc_to_broadcast, IComponent},
    message::{msg_types::Value, IMessage},
};
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};
use tracing::info;

use super::message::Message;
use super::ws2812rmt::WS2812RMT;

pub async fn hal(input: Option<mpsc::Receiver<Message>>, output: Option<mpsc::Sender<Message>>) {
    let (input_b_tx, _input_b_rx) = broadcast::channel(10);
    let (output_tx, output_rx) = mpsc::channel(10);

    let mut set: JoinSet<()> = JoinSet::new();

    let task_input = cmpbase_mpsc_to_broadcast::new(input, input_b_tx.clone());
    set.spawn(task_input);

    let _task_output = cmp_mpsc_to_mpsc::create().set_and_spawn(Some(output_rx), output);

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    // let nvs = EspDefaultNvsPartition::take().unwrap();

    // читаем кнопку с gpio9
    let button = PinDriver::input(peripherals.pins.gpio9).unwrap();
    set.spawn(gpio_read(
        input_b_tx.subscribe(),
        output_tx.clone(),
        button,
        |level| Message::Button(Value::new(*level)),
    ));

    // отправляем код цвета на LED
    let led = WS2812RMT::new(peripherals.pins.gpio8, peripherals.rmt.channel0).unwrap();
    set.spawn(ws2812(
        input_b_tx.subscribe(),
        output_tx.clone(),
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
