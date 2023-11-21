//! Компонент пересылает данные из одного канала в другой

use tracing::{error, info};

use rsiot_component_core::{Component, StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

async fn process<TMessage>(
    input: StreamInput<TMessage>,
    output: StreamOutput<TMessage>,
    _config: Config,
) where
    TMessage: IMessage,
{
    info!("cmp_mpsc_to_mpsc started");
    let mut input = match input {
        Some(val) => val,
        None => {
            let msg = "Input stream is None";
            error!("{:?}", msg);
            return;
        }
    };
    let output = match output {
        Some(val) => val,
        None => {
            let msg = "Output stream is None";
            error!("{:?}", msg);
            return;
        }
    };
    while let Some(msg) = input.recv().await {
        output.send(msg).await.unwrap();
    }
    error!("cmp_mpsc_to_mpsc stop");
}

#[derive(Clone, Debug)]
pub struct Config {}

pub fn create<TMessage>() -> Box<Component<TMessage, Config>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(Config {}, process);
    Box::new(cmp)
}
