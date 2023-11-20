//! Компонент для периодического генерирования сообщений

use tokio::time::{sleep, Duration, Instant};
use tracing::{error, info};

use rsiot_component_core::{Component, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

async fn cmp_inject_periodic<TMessage, TFnPeriodic>(
    _input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    mut config: Config<TMessage, TFnPeriodic>,
) where
    TMessage: IMessage,
    TFnPeriodic: FnMut() -> Vec<TMessage>,
{
    info!("cmp_inject_periodic stop started");
    let output = match output {
        Some(val) => val,
        None => {
            let msg = "Input stream is None";
            error!("{:?}", msg);
            return;
        }
    };
    loop {
        let begin = Instant::now();
        let msgs = (config.fn_periodic)();
        for msg in msgs {
            output.send(msg).await.unwrap();
        }
        let time_to_sleep = config.period - begin.elapsed();
        sleep(time_to_sleep).await;
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

pub fn create<TMessage, TFnPeriodic>(
    config: Config<TMessage, TFnPeriodic>,
) -> Box<Component<TMessage, Config<TMessage, TFnPeriodic>>>
where
    TMessage: IMessage + 'static,
    TFnPeriodic: FnMut() -> Vec<TMessage> + Send + 'static,
{
    let component = Component::new(config, cmp_inject_periodic);
    Box::new(component)
}
