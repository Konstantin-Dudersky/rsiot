use futures::{SinkExt, stream::SplitSink};
use tokio::{net::TcpStream, sync::broadcast};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message as TungsteniteMessage};
use tracing::{debug, trace};

use crate::{
    components::cmp_websocket_server::ServerToClientCache,
    components_config::websocket_server::WebsocketMessage, serde_utils::SerdeAlg,
};

pub struct SendToClient<TServerToClient> {
    pub input: broadcast::Receiver<TServerToClient>,
    pub websocket_write: SplitSink<WebSocketStream<TcpStream>, TungsteniteMessage>,
    pub cache: ServerToClientCache<TServerToClient>,
    pub serde_alg: SerdeAlg,
}

impl<TServerToClient> SendToClient<TServerToClient>
where
    TServerToClient: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        // Отправляем сообщения из кэша
        {
            let cache = self.cache.lock().await;
            for s2c in cache.values() {
                let text = create_text_from_msg(s2c, &self.serde_alg)?;
                self.websocket_write.send(text).await?;
            }
        }

        while let Ok(s2c) = self.input.recv().await {
            let text = create_text_from_msg(&s2c, &self.serde_alg)?;
            self.websocket_write.send(text).await?;
        }
        self.websocket_write.close().await?;
        debug!("Internal channel for sending to client closed");
        Ok(())
    }
}

#[allow(clippy::result_large_err)]
fn create_text_from_msg<TServerToClient>(
    s2c: &TServerToClient,
    serde_alg: &SerdeAlg,
) -> super::Result<TungsteniteMessage>
where
    TServerToClient: WebsocketMessage,
{
    trace!("Send message to client: {:?}", s2c);
    let s2c_bytes = serde_alg.serialize(s2c)?;
    let text = TungsteniteMessage::Binary(s2c_bytes);
    Ok(text)
}
