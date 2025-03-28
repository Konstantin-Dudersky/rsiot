use tokio::sync::mpsc;
use tracing::warn;

use crate::{
    components_config::{websocket_client::FnServerToClient, websocket_general::WebsocketMessage},
    message::{Message, MsgDataBound},
    serde_utils::SerdeAlg,
};

pub struct ServerToClient<TMsg, TServerToClient> {
    pub input: mpsc::Receiver<Vec<u8>>,
    pub output: mpsc::Sender<Message<TMsg>>,
    pub output_connection_state: mpsc::Sender<bool>,
    pub fn_output: FnServerToClient<TMsg, TServerToClient>,
    pub serde_alg: SerdeAlg,
}

impl<TMsg, TServerToClient> ServerToClient<TMsg, TServerToClient>
where
    TMsg: MsgDataBound,
    TServerToClient: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        let mut conn_state_sended = false;

        while let Some(bytes) = self.input.recv().await {
            let s2c = self.serde_alg.deserialize(&bytes);
            let s2c = match s2c {
                Ok(v) => v,
                Err(e) => {
                    warn!("Deserialization error: {:?}", e);
                    continue;
                }
            };

            let msgs = (self.fn_output)(s2c);

            if !conn_state_sended {
                conn_state_sended = true;
                self.output_connection_state
                    .send(true)
                    .await
                    .map_err(|_| super::Error::TokioSyncMpsc)?;
            }

            for msg in msgs {
                self.output
                    .send(msg)
                    .await
                    .map_err(|_| super::Error::TokioSyncMpsc)?;
            }
        }

        Err(super::Error::TaskOutput)
    }
}
