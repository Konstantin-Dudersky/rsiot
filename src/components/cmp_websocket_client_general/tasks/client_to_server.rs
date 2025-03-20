use tokio::sync::mpsc;

use crate::{
    components_config::{websocket_client::FnClientToServer, websocket_general::WebsocketMessage},
    message::{Message, MsgDataBound},
};

pub struct ClientToServer<TMsg, TClientToServer>
where
    TClientToServer: WebsocketMessage,
{
    pub input: mpsc::Receiver<Message<TMsg>>,
    pub output: mpsc::Sender<String>,
    pub fn_input: FnClientToServer<TMsg, TClientToServer>,
}

impl<TMsg, TClientToServer> ClientToServer<TMsg, TClientToServer>
where
    TMsg: MsgDataBound,
    TClientToServer: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(msg) = self.input.recv().await {
            let c2s = (self.fn_input)(&msg);
            let Some(c2s) = c2s else { continue };

            let text = serde_json::to_string(&c2s)
                .map_err(|e| super::Error::Serialization(e.to_string()))?;

            self.output
                .send(text)
                .await
                .map_err(|_| super::Error::TokioSyncMpsc)?;
        }

        Err(super::Error::TaskInput)
    }
}
