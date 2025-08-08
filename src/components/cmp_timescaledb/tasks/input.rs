use tokio::sync::mpsc;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{super::config::FnInput, Error, InnerMessage, Result};

pub struct Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: CmpInOut<TMsg>,
    pub output: mpsc::Sender<InnerMessage>,
    pub fn_input: FnInput<TMsg>,
}

impl<TMsg> Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Ok(msg) = self.input.recv_input().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            let items = (self.fn_input)(&msg);
            let Some(items) = items else { continue };
            self.output
                .send(InnerMessage::Rows(items))
                .await
                .map_err(|_| Error::TokioMpsc)?;
        }
        Err(Error::TaskInputEnd)
    }
}
