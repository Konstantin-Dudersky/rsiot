use std::time::Duration;

use rppal::gpio::{Gpio, InputPin, Level, OutputPin};
use tokio::{task::JoinSet, time::sleep};

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::{Config, Error};

const INPUT_READ_DELAY: Duration = Duration::from_millis(100);

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let gpio = Gpio::new()?;

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    for input_config in config.inputs {
        let pin = gpio.get(input_config.pin_number)?.into_input();
        task_set.spawn(input_pin(pin, input_config.fn_output, in_out.clone()));
    }

    for output_config in config.outputs {
        let pin = gpio.get(output_config.pin_number)?.into_output();
        task_set.spawn(output_pin(
            pin,
            output_config.fn_input,
            in_out.clone(),
            output_config.is_low_triggered,
        ));
    }

    while let Some(res) = task_set.join_next().await {
        res??
    }
    Ok(())
}

/// Функция чтения одного входа
///
/// В данной реализации просто периодически считывает состояние. Если в библиотеке `rppal` появится
/// возможность ожидать переключения в точке await - нужно переделать
async fn input_pin<TMsg>(
    pin: InputPin,
    fn_output: fn(bool) -> Message<TMsg>,
    in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mut prev_level: Option<bool> = None;
    loop {
        let level_read = pin.read();
        let level = match level_read {
            Level::High => true,
            Level::Low => false,
        };
        match prev_level {
            // Функция исполняется в первый раз
            None => {
                prev_level = Some(level);
            }
            // Функция исполняется не в первый раз
            Some(prev_level_value) => {
                // Значение со входа не изменилось
                if prev_level_value == level {
                    sleep(INPUT_READ_DELAY).await;
                    continue;
                // Значение изменилось
                } else {
                    prev_level = Some(level)
                }
            }
        }
        let msg = (fn_output)(level);
        in_out.send_output(msg).await.map_err(Error::CmpOutput)?;
        sleep(INPUT_READ_DELAY).await;
    }
}

/// Функция записи одного выхода
async fn output_pin<TMsg>(
    mut pin: OutputPin,
    fn_input: fn(Message<TMsg>) -> Option<bool>,
    mut in_out: CmpInOut<TMsg>,
    is_low_triggered: bool,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    // Значение по-умолчанию
    if is_low_triggered {
        pin.set_high();
    } else {
        pin.set_low();
    }

    while let Ok(msg) = in_out.recv_input().await {
        let Some(control) = (fn_input)(msg) else {
            continue;
        };
        if is_low_triggered ^ control {
            pin.set_high();
        } else {
            pin.set_low();
        }
    }
    Ok(())
}
