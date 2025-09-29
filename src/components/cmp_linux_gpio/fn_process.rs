use tokio::task::JoinSet;

use crate::{
    executor::{MsgBusInput, MsgBusOutput, join_set_spawn},
    message::MsgDataBound,
};

use super::{
    COMPONENT_NAME, Config, Error,
    tasks::{GpioInput, GpioOutput},
};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    input: MsgBusInput<TMsg>,
    output: MsgBusOutput<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    for input in &config.gpio_input {
        let task = GpioInput {
            msgbus_output: output.clone(),
            config: input.clone(),
        };
        join_set_spawn(
            &mut task_set,
            format!("{COMPONENT_NAME} | gpio_input | {}", input.description),
            task.spawn(),
        );
    }

    for output in &config.gpio_output {
        let task = GpioOutput {
            msgbus_input: input.clone(),
            config: output.clone(),
        };
        join_set_spawn(
            &mut task_set,
            format!("{COMPONENT_NAME} | gpio_output | {}", output.description),
            task.spawn(),
        );
    }

    drop(input);
    drop(output);

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::TaskEnd)
}
