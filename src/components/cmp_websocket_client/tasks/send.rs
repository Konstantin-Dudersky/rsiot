use futures::{stream::SplitSink, SinkExt};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{
    tungstenite::Message as TungsteniteMessage, MaybeTlsStream, WebSocketStream,
};

pub struct Send {
    pub input: mpsc::Receiver<Vec<u8>>,
    pub websocket_write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteMessage>,
}

impl Send {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(bytes) = self.input.recv().await {
            let text = TungsteniteMessage::Binary(bytes);
            self.websocket_write
                .send(text)
                .await
                .map_err(|e| super::Error::TaskSend(e.to_string()))?;
        }

        Err(super::Error::TaskSend("Task ended".to_string()))
    }
}
