use futures::{StreamExt, stream::SplitStream};
use gloo::net::websocket::{Message, futures::WebSocket};
use tokio::sync::mpsc;
use tracing::trace;

pub struct Receive {
    pub websocket_read: SplitStream<WebSocket>,
    pub output: mpsc::Sender<Vec<u8>>,
}

impl Receive {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(text) = self.websocket_read.next().await {
            trace!("New message from Websocket server: {:?}", text);
            let text = match text {
                Ok(text) => text,
                Err(_) => continue,
            };
            let bytes = match text {
                Message::Text(value) => value.as_bytes().to_vec(),
                Message::Bytes(bytes) => bytes,
            };
            self.output
                .send(bytes)
                .await
                .map_err(|_| super::Error::TokioSyncMpscSend)?;
        }
        Err(super::Error::TaskReceive("Task end".to_string()))
    }
}
