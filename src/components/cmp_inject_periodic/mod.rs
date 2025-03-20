//! Компонент для периодического генерирования сообщений

use async_trait::async_trait;
use instant::Instant;
use tokio::time::Duration;

use crate::{
    executor::{sleep, CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, Message, MsgDataBound, ServiceBound},
};

/// Конфигурация cmp_inject_periodic
#[derive(Clone, Debug)]
pub struct Config<TMsg, TFnPeriodic>
where
    TMsg: Clone,
    TFnPeriodic: FnMut() -> Vec<Message<TMsg>> + Send + Sync,
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
impl<TMsg, TFnPeriodic, TService> IComponentProcess<Config<TMsg, TFnPeriodic>, TMsg, TService>
    for Component<Config<TMsg, TFnPeriodic>, TMsg, TService>
where
    TMsg: MsgDataBound,
    TFnPeriodic: FnMut() -> Vec<Message<TMsg>> + Send + Sync,
    TService: ServiceBound,
{
    async fn process(
        &self,
        config: Config<TMsg, TFnPeriodic>,
        in_out: CmpInOut<TMsg, TService>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config,
            in_out.clone_with_new_id("cmp_inject_periodic", AuthPermissions::FullAccess),
        )
        .await
    }
}

async fn fn_process<TMsg, TFnPeriodic, TService>(
    mut config: Config<TMsg, TFnPeriodic>,
    in_out: CmpInOut<TMsg, TService>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
    TFnPeriodic: FnMut() -> Vec<Message<TMsg>> + Send + Sync,
    TService: ServiceBound,
{
    loop {
        let begin = Instant::now();
        let msgs = (config.fn_periodic)();
        for msg in msgs {
            in_out
                .send_output(msg)
                .await
                .map_err(|err| ComponentError::Execution(err.to_string()))?;
        }
        let elapsed = begin.elapsed();
        let sleep_time = if config.period <= elapsed {
            Duration::from_millis(10)
        } else {
            config.period - elapsed
        };
        sleep(sleep_time).await;
    }
}

/// Компонент cmp_inject_periodic
pub type Cmp<TMessage, TFnPeriodic, TService> =
    Component<Config<TMessage, TFnPeriodic>, TMessage, TService>;
