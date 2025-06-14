use tokio::sync::mpsc;
use tracing::warn;

use crate::{
    components_config::{websocket_client::FnClientToServer, websocket_general::WebsocketMessage},
    message::{Message, MsgDataBound},
    serde_utils::SerdeAlg,
};

pub struct ClientToServer<TMsg, TClientToServer>
where
    TClientToServer: WebsocketMessage,
{
    pub input: mpsc::Receiver<Message<TMsg>>,
    pub output: mpsc::Sender<Vec<u8>>,
    pub fn_input: FnClientToServer<TMsg, TClientToServer>,
    pub serde_alg: SerdeAlg,
}

impl<TMsg, TClientToServer> ClientToServer<TMsg, TClientToServer>
where
    TMsg: MsgDataBound,
    TClientToServer: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            let c2s = (self.fn_input)(&msg);
            let Some(c2s) = c2s else { continue };

            let bytes = self.serde_alg.serialize(&c2s);
            let bytes = match bytes {
                Ok(v) => v,
                Err(e) => {
                    warn!("Serialization error: {:?}", e);
                    continue;
                }
            };

            self.output
                .send(bytes)
                .await
                .map_err(|_| super::Error::TokioSyncMpsc)?;
        }

        Err(super::Error::TaskInput)
    }
}
