use tokio::task::JoinSet;

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, Error, task_command::TaskCommand};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> Result<(), Error>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    for cmd in config.commands {
        let task = TaskCommand {
            msgbus_input: msgbus_linker.input(),
            msgbus_output: msgbus_linker.output(),
            config: cmd,
        };

        join_set_spawn(&mut task_set, "cmd", task.spawn());
    }

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::FnProcessEnd)
}
