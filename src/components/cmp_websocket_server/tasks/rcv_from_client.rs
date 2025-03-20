// Рассмотреть возможность замены на бинарный протокол

use futures::{stream::SplitStream, StreamExt};
use serde_json::from_str;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::WebSocketStream;
use tracing::{debug, trace};

use crate::components_config::websocket_server::WebsocketMessage;

pub struct RcvFromClient<TClientToServer> {
    pub output: mpsc::Sender<TClientToServer>,
    pub websocket_read: SplitStream<WebSocketStream<TcpStream>>,
}

impl<TClientToServer> RcvFromClient<TClientToServer>
where
    TClientToServer: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(data) = self.websocket_read.next().await {
            let data = data?.into_text()?;
            if data.is_empty() {
                break;
            }

            let c2s: TClientToServer = from_str(&data)?;
            trace!("New message from websocket client: {:?}", c2s);
            self.output
                .send(c2s)
                .await
                .map_err(|_| super::Error::TokioSyncMpsc)?;
        }
        debug!("Input stream from client closed");
        Ok(())
    }
}
