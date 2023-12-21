//! Взаимодействие с HAL ESP32

use esp_idf_svc::hal::{
    gpio::{Input, InputPin, Level, PinDriver},
    peripherals::Peripherals,
};
use rsiot::{
    component::{cmp_mpsc_to_mpsc, cmpbase_mpsc_to_broadcast, IComponent},
    message::msg_types::Value,
};
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use super::message::Message;

pub async fn hal(input: Option<mpsc::Receiver<Message>>, output: Option<mpsc::Sender<Message>>) {
    let (input_b_tx, _input_b_rx) = broadcast::channel(10);
    let (output_tx, output_rx) = mpsc::channel(10);

    let mut set: JoinSet<()> = JoinSet::new();

    let task_input = cmpbase_mpsc_to_broadcast::new(input, input_b_tx.clone());
    set.spawn(task_input);

    let _task_output = cmp_mpsc_to_mpsc::create().set_and_spawn(Some(output_rx), output);

    let peripherals = Peripherals::take().unwrap();

    // читаем кнопку с gpio9
    let button = PinDriver::input(peripherals.pins.gpio9).unwrap();
    set.spawn(gpio_read(
        input_b_tx.subscribe(),
        output_tx.clone(),
        button,
        |level| {
            let level = gpio_level_to_bool(level);
            let msg = Message::Button(Value::new(level));
            vec![msg]
        },
    ));

    // читаем кнопку с gpio10
    let button = PinDriver::input(peripherals.pins.gpio10).unwrap();
    set.spawn(gpio_read(
        input_b_tx.subscribe(),
        output_tx.clone(),
        button,
        |level| {
            let level = gpio_level_to_bool(level);
            let msg = Message::Button(Value::new(level));
            vec![msg]
        },
    ));

    // читаем кнопку с gpio2
    let button = PinDriver::input(peripherals.pins.gpio2).unwrap();
    set.spawn(gpio_read(
        input_b_tx.subscribe(),
        output_tx.clone(),
        button,
        |level| {
            let level = gpio_level_to_bool(level);
            let msg = Message::Button(Value::new(level));
            vec![msg]
        },
    ));

    while (set.join_next().await).is_some() {}
}

async fn gpio_read<'a, TPin, TMessage>(
    _input: broadcast::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    mut driver: PinDriver<'a, TPin, Input>,
    fn_output: fn(&Level) -> Vec<TMessage>,
) where
    TPin: InputPin,
{
    loop {
        driver.wait_for_any_edge().await.unwrap();
        let level = driver.get_level();
        let msgs = (fn_output)(&level);
        for msg in msgs {
            output.send(msg).await.unwrap();
        }
    }
}

fn gpio_level_to_bool(level: &Level) -> bool {
    match level {
        Level::Low => true,
        Level::High => false,
    }
}
