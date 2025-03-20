use futures::{stream::SplitStream, StreamExt};
use gloo::net::websocket::{futures::WebSocket, Message};
use tokio::sync::mpsc;
use tracing::trace;

pub struct Receive {
    pub websocket_read: SplitStream<WebSocket>,
    pub output: mpsc::Sender<String>,
}

impl Receive {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(text) = self.websocket_read.next().await {
            trace!("New message from Websocket server: {:?}", text);
            let text = match text {
                Ok(text) => text,
                Err(_) => continue,
            };
            let text = match text {
                Message::Text(value) => value,
                Message::Bytes(_) => todo!(),
            };

            self.output
                .send(text)
                .await
                .map_err(|_| super::Error::TokioSyncMpsc)?;
        }
        Err(super::Error::TaskReceive("Task end".to_string()))
    }
}
