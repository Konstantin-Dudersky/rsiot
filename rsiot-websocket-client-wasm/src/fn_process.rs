use std::time::Duration;

use futures_util::{stream::SplitStream, StreamExt};
use gloo::{
    net::websocket::{futures::WebSocket, Message},
    timers::future::sleep,
};
use tokio::task::JoinSet;
use tracing::{info, trace, warn};

use rsiot_component_core::{ComponentInput, ComponentOutput};
use rsiot_components_config::websocket_client::Config;
use rsiot_messages_core::IMessage;

use crate::error::Error;

type Result<TMessage> = std::result::Result<(), Error<TMessage>>;

pub async fn fn_process<TMessage>(
    config: Config<TMessage>,
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
) -> Result<TMessage>
where
    TMessage: IMessage + 'static,
{
    info!("Starting");
    loop {
        let result = task_main(config.clone(), input.resubscribe(), output.clone()).await;
        warn!("End with resilt: {:?}", result);
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage>(
    config: Config<TMessage>,
    _input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
) -> Result<TMessage>
where
    TMessage: IMessage + 'static,
{
    let url = config.url.to_string();
    let ws = WebSocket::open(&url).map_err(|err| Error::Connect(err))?;
    info!("Connection to websocket server established");
    let (_write, read) = ws.split();

    let mut task_set: JoinSet<Result<TMessage>> = JoinSet::new();
    task_set.spawn_local(task_output(config, output, read));

    while let Some(task_result) = task_set.join_next().await {
        task_result??
    }
    Ok(())
}

async fn task_output<TMessage>(
    config: Config<TMessage>,
    output: ComponentOutput<TMessage>,
    mut read_stream: SplitStream<WebSocket>,
) -> Result<TMessage>
where
    TMessage: IMessage,
{
    while let Some(text) = read_stream.next().await {
        trace!("New message from Websocket server: {:?}", text);
        if let Ok(text) = text {
            let msg = match text {
                Message::Text(value) => value,
                Message::Bytes(_) => todo!(),
            };
            let msgs = (config.fn_output)(&msg).map_err(|err| Error::FnOutput(err))?;
            for msg in msgs {
                output.send(msg).await?;
            }
        };
    }
    Ok(())
}
