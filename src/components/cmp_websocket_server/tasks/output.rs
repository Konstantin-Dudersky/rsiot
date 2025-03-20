use tokio::sync::mpsc;

use crate::{
    components_config::websocket_server::{FnOutput, WebsocketMessage},
    message::{Message, MsgDataBound},
};

pub struct Output<TMsg, TClientToServer>
where
    TMsg: MsgDataBound,
    TClientToServer: WebsocketMessage,
{
    pub input: mpsc::Receiver<TClientToServer>,
    pub output: mpsc::Sender<Message<TMsg>>,
    pub fn_output: FnOutput<TMsg, TClientToServer>,
}

impl<TMsg, TClientToServer> Output<TMsg, TClientToServer>
where
    TMsg: MsgDataBound,
    TClientToServer: WebsocketMessage,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        while let Some(c2s) = self.input.recv().await {
            let msgs = (self.fn_output)(c2s);
            for msg in msgs {
                self.output
                    .send(msg)
                    .await
                    .map_err(|_| super::Error::TokioSyncMpsc)?;
            }
        }
        Err(super::Error::TaskEndOutput)
    }
}
