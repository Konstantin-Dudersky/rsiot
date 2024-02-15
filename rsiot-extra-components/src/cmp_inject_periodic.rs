//! Компонент для периодического генерирования сообщений

use async_trait::async_trait;
use tokio::time::{sleep, Duration, Instant};
use tracing::debug;

use rsiot_component_core::{
    cmp_set_component_id, Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::IMessage;

#[derive(Clone, Debug)]
pub struct Config<TMessage, TFnPeriodic>
where
    TMessage: IMessage,
    TFnPeriodic: FnMut() -> Vec<TMessage>,
{
    /// Период вызова
    pub period: Duration,
    /// Функция для генерирования сообщений
    pub fn_periodic: TFnPeriodic,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage, TFnPeriodic> IComponentProcess<Config<TMessage, TFnPeriodic>, TMessage>
    for Component<Config<TMessage, TFnPeriodic>, TMessage>
where
    TMessage: IMessage,
    TFnPeriodic: FnMut() -> Vec<TMessage> + Send + Sync,
{
    async fn process(
        &self,
        config: Config<TMessage, TFnPeriodic>,
        mut input: CmpInput<TMessage>,
        mut output: CmpOutput<TMessage>,
        cache: Cache<TMessage>,
    ) -> Result<(), ComponentError> {
        cmp_set_component_id(&mut input, &mut output, "cmp_inject_periodic");
        process(config, input, output, cache).await
    }
}

async fn process<TMessage, TFnPeriodic>(
    mut config: Config<TMessage, TFnPeriodic>,
    _input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    _cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage,
    TFnPeriodic: FnMut() -> Vec<TMessage>,
{
    debug!("cmp_inject_periodic started");
    loop {
        let begin = Instant::now();
        let msgs = (config.fn_periodic)();
        for msg in msgs {
            output
                .send(msg)
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

pub type Cmp<TMessage, TFnPeriodic> = Component<Config<TMessage, TFnPeriodic>, TMessage>;
