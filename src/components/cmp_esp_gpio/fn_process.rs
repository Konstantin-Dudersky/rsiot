use esp_idf_svc::hal::gpio::{Level, PinDriver};
use tokio::task::JoinSet;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, ConfigGpioInput, ConfigGpioOutput};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut task_set = JoinSet::new();
    for config_input in config.inputs {
        task_set.spawn_local(gpio_input(config_input, in_out.clone()));
    }
    for config_output in config.outputs {
        task_set.spawn_local(gpio_output(config_output, in_out.clone()));
    }

    while let Some(res) = task_set.join_next().await {
        res.unwrap();
    }
    Ok(())
}

/// Функция чтения одного входа
async fn gpio_input<TMsg>(config_input: ConfigGpioInput<TMsg>, in_out: CmpInOut<TMsg>)
where
    TMsg: MsgDataBound,
{
    let mut pin = PinDriver::input(config_input.peripherals).unwrap();
    pin.set_pull(config_input.pull).unwrap();

    loop {
        let level = pin.get_level();
        let level = gpio_level_to_bool(&level);
        let msg = (config_input.fn_output)(level);
        in_out.send_output(msg).await.unwrap();
        pin.wait_for_any_edge().await.unwrap();
    }
}

/// Функция записи одного выхода
async fn gpio_output<TMsg>(config_output: ConfigGpioOutput<TMsg>, mut in_out: CmpInOut<TMsg>)
where
    TMsg: MsgDataBound,
{
    let mut pin = PinDriver::output(config_output.peripherals).unwrap();

    // Значение по-умолчанию
    if config_output.is_low_triggered {
        pin.set_high().unwrap();
    } else {
        pin.set_low().unwrap();
    }

    while let Ok(msg) = in_out.recv_input().await {
        let level = (config_output.fn_input)(msg);
        let Some(level) = level else { continue };
        if config_output.is_low_triggered ^ level {
            pin.set_high().unwrap();
        } else {
            pin.set_low().unwrap();
        }
    }
}

fn gpio_level_to_bool(level: &Level) -> bool {
    match level {
        Level::Low => true,
        Level::High => false,
    }
}
