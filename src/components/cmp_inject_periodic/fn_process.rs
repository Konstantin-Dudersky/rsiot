use tokio::{task::JoinSet, time::Duration};

use crate::{
    executor::{MsgBusLinker, Instant, MsgBusOutput, join_set_spawn, sleep},
    message::{Message, MsgDataBound},
};

use super::{Config, Error};

pub async fn fn_process<TMsg, TFnPeriodic>(
    config: Config<TMsg, TFnPeriodic>,
    msg_bus: MsgBusLinker<TMsg>,
) -> Result<(), Error>
where
    TMsg: 'static + MsgDataBound,
    TFnPeriodic: 'static + FnMut() -> Vec<TMsg> + Send + Sync,
{
    let mut task_set = JoinSet::new();

    let task = TaskInjectPeriodic {
        config,
        msgbus_output: msg_bus.output(),
    };
    join_set_spawn(&mut task_set, "cmp_inject_periodic", task.spawn());

    drop(msg_bus);

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}

struct TaskInjectPeriodic<TMsg, TFnPeriodic>
where
    TMsg: MsgDataBound,
    TFnPeriodic: FnMut() -> Vec<TMsg> + Send + Sync,
{
    config: Config<TMsg, TFnPeriodic>,
    msgbus_output: MsgBusOutput<TMsg>,
}
impl<TMsg, TFnPeriodic> TaskInjectPeriodic<TMsg, TFnPeriodic>
where
    TMsg: MsgDataBound,
    TFnPeriodic: FnMut() -> Vec<TMsg> + Send + Sync,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        loop {
            let begin = Instant::now();
            let msgs = (self.config.fn_periodic)();
            for msg in msgs {
                let msg = Message::new_custom(msg);
                self.msgbus_output
                    .send(msg)
                    .await
                    .map_err(|err| Error::TokioMpscSend(err.to_string()))?;
            }
            let elapsed = begin.elapsed();
            let sleep_time = if self.config.period <= elapsed {
                Duration::from_millis(10)
            } else {
                self.config.period - elapsed
            };
            sleep(sleep_time).await;
        }
    }
}
