//! Компонент для периодического генерирования сообщений

use tokio::time::{sleep, Duration, Instant};
use tracing::debug;

use rsiot_component_core::{Cache, Component, ComponentError, ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

async fn fn_process<TMessage, TFnPeriodic>(
    _input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    mut config: Config<TMessage, TFnPeriodic>,
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

pub fn new<TMessage, TFnPeriodic>(
    config: Config<TMessage, TFnPeriodic>,
) -> Box<Component<TMessage, Config<TMessage, TFnPeriodic>>>
where
    TMessage: IMessage + 'static,
    TFnPeriodic: FnMut() -> Vec<TMessage> + Send + 'static,
{
    let component = Component::new(config, fn_process);
    Box::new(component)
}
