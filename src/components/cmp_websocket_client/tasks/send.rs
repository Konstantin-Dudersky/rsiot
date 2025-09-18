use bytes::Bytes;
use futures::{SinkExt, stream::SplitSink};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, tungstenite::Message as TungsteniteMessage,
};

pub struct Send {
    pub input: mpsc::Receiver<Vec<u8>>,
    pub websocket_write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, TungsteniteMessage>,
}

impl Send {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(bytes) = self.input.recv().await {
            let bytes = Bytes::from_owner(bytes);
            let text = TungsteniteMessage::Binary(bytes);
            self.websocket_write
                .send(text)
                .await
                .map_err(|e| super::Error::TaskSend(e.to_string()))?;
        }

        Err(super::Error::TaskSend("Task ended".to_string()))
    }
}
