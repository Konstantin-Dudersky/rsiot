//! Взаимодействие с HAL ESP32

use esp_idf_svc::hal::{
    gpio::{Input, InputPin, PinDriver},
    peripherals::Peripherals,
};
use rsiot::component::cmpbase_mpsc_to_broadcast;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

pub async fn hal<TMessage>(input: mpsc::Receiver<TMessage>, output: mpsc::Sender<TMessage>) {
    let mut set: JoinSet<()> = JoinSet::new();

    let peripherals = Peripherals::take().unwrap();
    let mut button = PinDriver::input(peripherals.pins.gpio9).unwrap();

    while (set.join_next().await).is_some() {}
}

async fn read_di<'a, TPin, TMessage>(
    input: broadcast::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    mut driver: PinDriver<'a, TPin, Input>,
    fn_output: fn() -> Vec<TMessage>,
) where
    TPin: InputPin,
{
    loop {
        driver.wait_for_any_edge().await.unwrap();
        println!("{:?}", driver.get_level());
    }
}
