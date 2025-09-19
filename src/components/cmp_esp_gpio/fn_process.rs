use tokio::task::JoinSet;

use crate::{
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, Error, task_gpio_input::GpioInput, task_gpio_output::GpioOutput};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();
    for config_input in config.inputs {
        let task = GpioInput {
            in_out: in_out.clone(),
            config_input,
        };
        join_set_spawn(&mut task_set, "gpio_input", task.spawn());
    }
    for config_output in config.outputs {
        let task = GpioOutput {
            in_out: in_out.clone(),
            config_output,
        };
        join_set_spawn(&mut task_set, "gpio_output", task.spawn());
    }

    while let Some(res) = task_set.join_next().await {
        res??;
    }
    Ok(())
}
