use futures::{SinkExt, stream::SplitSink};
use gloo::net::websocket::{Message, futures::WebSocket};
use tokio::sync::broadcast;

pub struct Send {
    pub input: broadcast::Receiver<Vec<u8>>,
    pub websocket_write: SplitSink<WebSocket, Message>,
}

impl Send {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Ok(bytes) = self.input.recv().await {
            let text = Message::Bytes(bytes);
            self.websocket_write
                .send(text)
                .await
                .map_err(|e| super::Error::TaskSend(e.to_string()))?;
        }
        Err(super::Error::TaskSend("Task ended".to_string()))
    }
}
