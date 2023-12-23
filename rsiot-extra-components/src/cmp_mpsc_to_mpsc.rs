//! Компонент пересылает данные из одного канала в другой

use tracing::{debug, error};

use rsiot_component_core::{Component, Input, Output};
use rsiot_messages_core::IMessage;

async fn process<TMessage>(mut input: Input<TMessage>, output: Output<TMessage>, _config: Config)
where
    TMessage: IMessage,
{
    debug!("cmp_mpsc_to_mpsc started");
    while let Ok(msg) = input.recv().await {
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
