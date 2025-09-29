use tokio::task::JoinSet;
use tracing::{debug, error, info, trace, warn};

use tracing::Level;

use crate::executor::{CmpInOut, MsgBusInput};
use crate::{executor::join_set_spawn, message::MsgDataBound};

use super::{Config, Error};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: CmpInOut<TMsg>,
) -> Result<(), Error>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set = JoinSet::new();

    let task = TaskLogger {
        config,
        input: msgbus_linker.input(),
    };
    join_set_spawn(&mut task_set, "cmp_logger", task.spawn());

    drop(msgbus_linker);

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::TaskEnd)
}

struct TaskLogger<TMsg>
where
    TMsg: MsgDataBound,
{
    pub config: Config<TMsg>,
    pub input: MsgBusInput<TMsg>,
}
impl<TMsg> TaskLogger<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Ok(msg) = self.input.recv().await {
            let text = (self.config.fn_input)(msg);
            // ошибка сериализации
            let Ok(text) = text else {
                warn!("Logger Error: {:?}", text);
                continue;
            };
            // фильтрация
            let Some(text) = text else { continue };
            match self.config.level {
                Level::TRACE => trace!("{text}"),
                Level::DEBUG => debug!("{text}"),
                Level::INFO => info!("{text}"),
                Level::WARN => warn!("{text}"),
                Level::ERROR => error!("{text}"),
            }
        }

        Err(Error::TaskEnd)
    }
}
