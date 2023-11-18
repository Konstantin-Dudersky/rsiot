use tokio::{
    spawn,
    sync::mpsc,
    time::{sleep, Duration, Instant},
};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;
use tracing::error;

/// Компонент для периодического генерирования сообщений
pub async fn component_inject_periodic<TMessage, TFnPeriodic>(
    stream_output: mpsc::Sender<TMessage>,
    period: Duration,
    mut fn_periodic: TFnPeriodic,
) where
    TMessage: IMessage,
    TFnPeriodic: FnMut() -> Vec<TMessage>,
{
    loop {
        let begin = Instant::now();
        let msgs = (fn_periodic)();
        for msg in msgs {
            stream_output.send(msg).await.unwrap();
        }
        let time_to_sleep = period - begin.elapsed();
        sleep(time_to_sleep).await;
    }
}

/// Компонент для периодического генерирования сообщений
pub async fn component_inject_periodic2<TMessage, TFnPeriodic>(
    input: Option<StreamInput<TMessage>>,
    mut output: Option<StreamOutput<TMessage>>,
    mut config: CompInjectPeriodicConfig<TMessage, TFnPeriodic>,
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
pub struct CompInjectPeriodicConfig<TMessage, TFnPeriodic>
where
    TMessage: IMessage,
    TFnPeriodic: FnMut() -> Vec<TMessage>,
{
    pub period: Duration,
    pub fn_periodic: TFnPeriodic,
}

use rsiot_component_core::Component;

pub fn create_inject_periodic<TMessage, TFnPeriodic>(
    config: CompInjectPeriodicConfig<TMessage, TFnPeriodic>,
) -> Box<Component<TMessage, CompInjectPeriodicConfig<TMessage, TFnPeriodic>>>
where
    TMessage: IMessage + 'static,
    TFnPeriodic: FnMut() -> Vec<TMessage> + Send + 'static,
{
    let component = Component::new(config, component_inject_periodic2);
    Box::new(component)
}
