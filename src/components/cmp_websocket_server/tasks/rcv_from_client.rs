use futures::{StreamExt, stream::SplitStream};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::WebSocketStream;
use tracing::{debug, trace};

use crate::{
    components_config::websocket_server::{WebsocketMessage, WsData},
    serde_utils::SerdeAlg,
};

pub struct RcvFromClient<TClientToServer>
where
    TClientToServer: WebsocketMessage,
{
    pub output: mpsc::Sender<WsData<TClientToServer>>,
    pub websocket_read: SplitStream<WebSocketStream<TcpStream>>,
    pub serde_alg: SerdeAlg,
    pub client_id: String,
}

impl<TClientToServer> RcvFromClient<TClientToServer>
where
    TClientToServer: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(data) = self.websocket_read.next().await {
            let data = data?.into_data();
            if data.is_empty() {
                break;
            }

            let c2s: TClientToServer = self.serde_alg.deserialize(&data)?;
            trace!(
                "New message from websocket client: {:?}; client_id: {}",
                c2s, self.client_id
            );
            let c2s = WsData {
                client_id: Some(self.client_id.clone()),
                data: c2s,
            };

            self.output
                .send(c2s)
                .await
                .map_err(|_| super::Error::TokioSyncMpsc)?;
        }
        debug!("Input stream from client closed");
        Ok(())
    }
}
