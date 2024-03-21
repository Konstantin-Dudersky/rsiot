use rppal::gpio::{Gpio, Level, OutputPin, Trigger};
use tokio::task::JoinSet;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::Config;

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let gpio = Gpio::new().unwrap();

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    for input_config in config.inputs {
        let mut pin = gpio.get(input_config.pin_number).unwrap().into_input();
        let in_out_clone = in_out.clone();
        pin.set_async_interrupt(Trigger::Both, move |level| {
            let level = match level {
                Level::High => true,
                Level::Low => false,
            };
            let msg = (input_config.fn_output)(level);
            in_out_clone.send_output_blocking(msg).unwrap();
        })
        .unwrap()
    }

    for output_config in config.outputs {
        let pin = gpio.get(output_config.pin_number).unwrap().into_output();
        task_set.spawn(output_pin(pin, output_config.fn_input, in_out.clone()));
    }

    while let Some(res) = task_set.join_next().await {
        res.unwrap().unwrap()
    }
    Ok(())
}

async fn output_pin<TMsg>(
    mut pin: OutputPin,
    fn_input: fn(Message<TMsg>) -> Option<bool>,
    mut in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    while let Ok(msg) = in_out.recv_input().await {
        let Some(control) = (fn_input)(msg) else {
            continue;
        };
        match control {
            true => pin.set_high(),
            false => pin.set_low(),
        }
    }
    Ok(())
}
