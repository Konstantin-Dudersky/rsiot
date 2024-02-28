//! Компонент для периодического генерирования сообщений

use async_trait::async_trait;
use tokio::time::{sleep, Duration, Instant};
use tracing::debug;

use rsiot_component_core::{
    cmp_set_component_name, Cache, CmpInput, CmpOutput, Component, ComponentError,
    IComponentProcess,
};
use rsiot_messages_core::{Message, MsgDataBound};

#[derive(Clone, Debug)]
pub struct Config<TMsg, TFnPeriodic>
where
    TMsg: Clone,
    TFnPeriodic: FnMut() -> Vec<Message<TMsg>>,
{
    /// Период вызова
    pub period: Duration,
    /// Функция для генерирования сообщений
    pub fn_periodic: TFnPeriodic,
}

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TFnPeriodic> IComponentProcess<Config<TMsg, TFnPeriodic>, TMsg>
    for Component<Config<TMsg, TFnPeriodic>, TMsg>
where
    TMsg: MsgDataBound,
    TFnPeriodic: FnMut() -> Vec<Message<TMsg>> + Send + Sync,
{
    async fn process(
        &self,
        config: Config<TMsg, TFnPeriodic>,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        cmp_set_component_name(&mut input, &mut output, "cmp_inject_periodic");
        process(config, input, output, cache).await
    }
}

async fn process<TMsg, TFnPeriodic>(
    mut config: Config<TMsg, TFnPeriodic>,
    _input: CmpInput<TMsg>,
    output: CmpOutput<TMsg>,
    _cache: Cache<TMsg>,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
    TFnPeriodic: FnMut() -> Vec<Message<TMsg>>,
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
