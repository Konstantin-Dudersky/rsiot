//! Компонент для периодического генерирования сообщений

use tokio::time::{sleep, Duration, Instant};

use rsiot_component_core::{StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

async fn cmp_inject_periodic<TMessage, TFnPeriodic>(
    _input: StreamInput<TMessage>,
    mut output: StreamOutput<TMessage>,
    mut config: Config<TMessage, TFnPeriodic>,
) where
    TMessage: IMessage,
    TFnPeriodic: FnMut() -> Vec<TMessage>,
{
    let stream_output = output.take().unwrap();
    loop {
        let begin = Instant::now();
        let msgs = (config.fn_periodic)();
        for msg in msgs {
            stream_output.send(msg).await.unwrap();
        }
        let time_to_sleep = config.period - begin.elapsed();
        sleep(time_to_sleep).await;
    }
}

#[derive(Clone)]
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

use rsiot_component_core::Component;

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
