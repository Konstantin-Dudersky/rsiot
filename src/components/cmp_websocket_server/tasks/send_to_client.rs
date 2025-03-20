// Рассмотреть возможность замены на бинарный протокол

use futures::{stream::SplitSink, SinkExt};
use serde_json::to_string;
use tokio::{net::TcpStream, sync::broadcast};
use tokio_tungstenite::{tungstenite::Message as TungsteniteMessage, WebSocketStream};
use tracing::{debug, trace};

use crate::{
    components::cmp_websocket_server::ServerToClientCache,
    components_config::websocket_server::WebsocketMessage,
};

pub struct SendToClient<TServerToClient> {
    pub input: broadcast::Receiver<TServerToClient>,
    pub websocket_write: SplitSink<WebSocketStream<TcpStream>, TungsteniteMessage>,
    pub cache: ServerToClientCache<TServerToClient>,
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
                let json = to_string(&s2c)?;
                trace!("Send message to client: {:?}", json);
                let text = TungsteniteMessage::Text(json);
                self.websocket_write.send(text).await?;
            }
        }

        while let Ok(s2c) = self.input.recv().await {
            let json = to_string(&s2c)?;
            trace!("Send message to client: {:?}", json);
            let text = TungsteniteMessage::Text(json);
            self.websocket_write.send(text).await?;
        }
        self.websocket_write.close().await.unwrap();
        debug!("Internal channel for sending to client closed");
        Ok(())
    }
}
