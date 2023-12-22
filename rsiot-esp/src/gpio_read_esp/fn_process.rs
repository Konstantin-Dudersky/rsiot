use esp_idf_svc::hal::{
    gpio::{Input, InputPin, PinDriver},
    peripherals::Peripherals,
};
use tokio::{
    spawn,
    sync::mpsc,
    time::{sleep, Duration},
};
use tracing::{debug, error, info, trace, warn};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_extra_components::cmp_mpsc_to_mpsc;
use rsiot_messages_core::IMessage;

use super::config::Config;

pub async fn fn_process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    config: Config<TMessage>,
) where
    TMessage: IMessage + 'static,
{
    let (output_main_task_tx, output_main_task_rx) = mpsc::channel::<TMessage>(10);

    // TODO - объединять данные со входа и внутренней задачи
    let _task_output = cmp_mpsc_to_mpsc::create().set_and_spawn(Some(output_main_task_rx), output);

    let peripherals = Peripherals::take().unwrap();

    let button = PinDriver::input(peripherals.pins.gpio9).unwrap();
    let task = spawn(read_di(button));

    let button2 = PinDriver::input(peripherals.pins.gpio10).unwrap();
    let task2 = spawn(read_di(button2));

    task.await.unwrap();
}

async fn read_di<'a, TPin>(mut driver: PinDriver<'a, TPin, Input>)
where
    TPin: InputPin,
{
    loop {
        driver.wait_for_any_edge().await.unwrap();
        println!("{:?}", driver.get_level());
    }
}
