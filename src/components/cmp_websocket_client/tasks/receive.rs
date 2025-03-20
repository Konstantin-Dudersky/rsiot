use futures::{stream::SplitStream, StreamExt};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub struct Receive {
    pub websocket_read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    pub output: mpsc::Sender<String>,
}

impl Receive {
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(msg) = self.websocket_read.next().await {
            let data = msg
                .map_err(|e| super::Error::TaskReceive(e.to_string()))?
                .into_text()
                .map_err(|e| super::Error::TaskReceive(e.to_string()))?;

            self.output
                .send(data)
                .await
                .map_err(|_| super::Error::TokioSyncMpsc)?;
        }
        Err(super::Error::TaskReceive("Task end".to_string()))
    }
}
