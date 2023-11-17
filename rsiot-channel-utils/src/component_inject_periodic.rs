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

pub struct InjectPeriodic<TMessage> {
    stream_input: Option<StreamInput<TMessage>>,
    stream_output: Option<StreamOutput<TMessage>>,
    config: fn() -> Vec<TMessage>,
    period: Duration,
}

impl<TMessage> InjectPeriodic<TMessage> {
    pub fn new(config: fn() -> Vec<TMessage>, period: Duration) -> Box<Self> {
        Self {
            stream_input: None,
            stream_output: None,
            config,
            period,
        }
        .into()
    }
}

impl<TMessage> IComponent<TMessage> for InjectPeriodic<TMessage>
where
    TMessage: IMessage + 'static,
{
    fn set_stream_input(&mut self, stream_input: StreamInput<TMessage>) {
        self.stream_input = Some(stream_input);
    }

    fn set_stream_output(&mut self, stream_output: StreamOutput<TMessage>) {
        self.stream_output = Some(stream_output);
    }

    fn spawn(&mut self) -> tokio::task::JoinHandle<()> {
        let config = self.config.clone();
        let period = self.period.clone();
        let stream_output = self.stream_output.take().unwrap();
        spawn(async move {
            loop {
                let begin = Instant::now();
                let msgs = (config)();
                for msg in msgs {
                    let res = stream_output.send(msg).await;
                    if let Err(err) = res {
                        error!("Send error: {:?}", err);
                    }
                }
                let time_to_sleep = period - begin.elapsed();
                sleep(time_to_sleep).await;
            }
        })
    }
}
