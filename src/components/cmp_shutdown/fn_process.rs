use tokio::task::JoinSet;

use crate::{
    executor::{MsgBusInput, MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{COMPONENT_NAME, Config, Error};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set = JoinSet::new();

    let task = Task {
        input: msgbus_linker.input(),
        fn_input: config.fn_input,
    };

    join_set_spawn(&mut task_set, COMPONENT_NAME, task.spawn());

    msgbus_linker.close();

    while task_set.join_next().await.is_some() {}

    Ok(())
}

struct Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub fn_input: fn(TMsg) -> bool,
}

impl<TMsg> Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };

            let result = (self.fn_input)(msg);

            match result {
                true => {
                    return Ok(());
                }
                false => {
                    continue;
                }
            }
        }

        Err(Error::TaskEnd)
    }
}
