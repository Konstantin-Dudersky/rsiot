//! Компонент для периодического генерирования сообщений

use async_trait::async_trait;
use instant::Instant;
use tokio::{task::JoinSet, time::Duration};

use crate::{
    executor::{join_set_spawn, sleep, CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, Message, MsgDataBound},
};

/// Конфигурация cmp_inject_periodic
#[derive(Clone, Debug)]
pub struct Config<TMsg, TFnPeriodic>
where
    TMsg: Clone,
    TFnPeriodic: FnMut() -> Vec<TMsg> + Send + Sync,
{
    /// Период вызова
    pub period: Duration,

    /// Функция для генерирования сообщений
    ///
    /// Тип данных - `FnMut() -> Vec<Message<TMsg>>`
    pub fn_periodic: TFnPeriodic,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TFnPeriodic> IComponentProcess<Config<TMsg, TFnPeriodic>, TMsg>
    for Component<Config<TMsg, TFnPeriodic>, TMsg>
where
    TMsg: 'static + MsgDataBound,
    TFnPeriodic: 'static + FnMut() -> Vec<TMsg> + Send + Sync,
{
    async fn process(
        &self,
        config: Config<TMsg, TFnPeriodic>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config,
            in_out.clone_with_new_id("cmp_inject_periodic", AuthPermissions::FullAccess),
        )
        .await
    }
}

async fn fn_process<TMsg, TFnPeriodic>(
    config: Config<TMsg, TFnPeriodic>,
    in_out: CmpInOut<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: 'static + MsgDataBound,
    TFnPeriodic: 'static + FnMut() -> Vec<TMsg> + Send + Sync,
{
    let mut task_set = JoinSet::new();

    let task = TaskInjectPeriodic { config, in_out };
    join_set_spawn(&mut task_set, "cmp_inject_periodic", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res.unwrap()?;
    }

    // loop {
    //     let begin = Instant::now();
    //     let msgs = (config.fn_periodic)();
    //     for msg in msgs {
    //         let msg = Message::new_custom(msg);
    //         in_out
    //             .send_output(msg)
    //             .await
    //             .map_err(|err| ComponentError::Execution(err.to_string()))?;
    //     }
    //     let elapsed = begin.elapsed();
    //     let sleep_time = if config.period <= elapsed {
    //         Duration::from_millis(10)
    //     } else {
    //         config.period - elapsed
    //     };
    //     sleep(sleep_time).await;
    // }

    Ok(())
}

struct TaskInjectPeriodic<TMsg, TFnPeriodic>
where
    TMsg: MsgDataBound,
    TFnPeriodic: FnMut() -> Vec<TMsg> + Send + Sync,
{
    config: Config<TMsg, TFnPeriodic>,
    in_out: CmpInOut<TMsg>,
}
impl<TMsg, TFnPeriodic> TaskInjectPeriodic<TMsg, TFnPeriodic>
where
    TMsg: MsgDataBound,
    TFnPeriodic: FnMut() -> Vec<TMsg> + Send + Sync,
{
    pub async fn spawn(mut self) -> Result<(), ComponentError> {
        loop {
            let begin = Instant::now();
            let msgs = (self.config.fn_periodic)();
            for msg in msgs {
                let msg = Message::new_custom(msg);
                self.in_out
                    .send_output(msg)
                    .await
                    .map_err(|err| ComponentError::Execution(err.to_string()))?;
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

/// Компонент cmp_inject_periodic
pub type Cmp<TMessage, TFnPeriodic> = Component<Config<TMessage, TFnPeriodic>, TMessage>;
