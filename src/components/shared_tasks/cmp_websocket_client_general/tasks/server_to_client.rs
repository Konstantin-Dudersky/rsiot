use tokio::sync::mpsc;
use tracing::warn;

use crate::{
    components_config::{websocket_client::FnServerToClient, websocket_general::WebsocketMessage},
    executor::MsgBusOutput,
    message::{Message, MsgDataBound},
    serde_utils::SerdeAlg,
};

pub struct ServerToClient<TMsg, TServerToClient>
where
    TMsg: MsgDataBound,
{
    pub input: mpsc::Receiver<Vec<u8>>,
    pub output: MsgBusOutput<TMsg>,
    pub fn_output: FnServerToClient<TMsg, TServerToClient>,
    pub serde_alg: SerdeAlg,
}

impl<TMsg, TServerToClient> ServerToClient<TMsg, TServerToClient>
where
    TMsg: MsgDataBound,
    TServerToClient: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
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

            for msg in msgs {
                let msg = Message::new_custom(msg);
                self.output
                    .send(msg)
                    .await
                    .map_err(|_| super::Error::TokioSyncMpscSend)?;
            }
        }

        Err(super::Error::TaskOutput)
    }
}
