use tokio::{spawn, sync::mpsc};
use tracing::{debug, error, info, trace, warn, Level};

use rsiot_component_core::{IComponent, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

/// Компонент для логгирования сообщений
pub async fn component_logger<TMessage>(
    mut stream_input: mpsc::Receiver<TMessage>,
    stream_output: Option<mpsc::Sender<TMessage>>,
    level: Level,
) where
    TMessage: IMessage,
{
    while let Some(msg) = stream_input.recv().await {
        match level {
            Level::TRACE => trace!("{:?}", msg),
            Level::DEBUG => debug!("{:?}", msg),
            Level::INFO => info!("{:?}", msg),
            Level::WARN => warn!("{:?}", msg),
            Level::ERROR => error!("{:?}", msg),
        }
        match &stream_output {
            Some(stream) => stream.send(msg).await.unwrap(),
            None => (),
        }
    }
}

pub struct Logger<TMessage> {
    stream_input: Option<StreamInput<TMessage>>,
    stream_output: Option<StreamOutput<TMessage>>,
}

impl<TMessage> Logger<TMessage> {
    pub fn new() -> Box<Self> {
        Self {
            stream_input: None,
            stream_output: None,
        }
        .into()
    }
}

impl<TMessage> IComponent<TMessage> for Logger<TMessage>
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
        info!("spawn logger");
        let mut stream_input = self.stream_input.take().unwrap();
        spawn(async move {
            while let Some(msg) = stream_input.recv().await {
                info!("New message: {:?}", msg);
            }
        })
    }
}
