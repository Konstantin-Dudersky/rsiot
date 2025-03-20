use tokio::sync::mpsc;

use crate::{
    components_config::{websocket_client::FnServerToClient, websocket_general::WebsocketMessage},
    message::{Message, MsgDataBound},
};

pub struct ServerToClient<TMsg, TServerToClient> {
    pub input: mpsc::Receiver<String>,
    pub output: mpsc::Sender<Message<TMsg>>,
    pub output_connection_state: mpsc::Sender<bool>,
    pub fn_output: FnServerToClient<TMsg, TServerToClient>,
}

impl<TMsg, TServerToClient> ServerToClient<TMsg, TServerToClient>
where
    TMsg: MsgDataBound,
    TServerToClient: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        let mut conn_state_sended = false;

        while let Some(text) = self.input.recv().await {
            let s2c: TServerToClient = serde_json::from_str(&text)
                .map_err(|e| super::Error::Deserialization(e.to_string()))?;
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
