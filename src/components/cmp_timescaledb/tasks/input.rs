use tokio::sync::mpsc;

use crate::{
    executor::{CheckCapacity, MsgBusInput},
    message::MsgDataBound,
};

use super::{super::config::FnInput, COMPONENT_NAME, Error, InnerMessage, Result};

pub struct Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_input: MsgBusInput<TMsg>,
    pub output: mpsc::Sender<InnerMessage>,
    pub fn_input: FnInput<TMsg>,
}

impl<TMsg> Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        let desc = format!("{COMPONENT_NAME} | task Input | channel output");

        while let Ok(msg) = self.msgbus_input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            let items = (self.fn_input)(&msg);
            let Some(items) = items else { continue };
            self.output
                .check_capacity(0.2, &desc)
                .send(InnerMessage::Rows(items))
                .await
                .map_err(|_| Error::TokioMpsc { task_name: "Input" })?;
        }
        Err(Error::TaskInputEnd)
    }
}
