use tokio::sync::mpsc;

use crate::{
    executor::{CheckCapacity, MsgBusInput},
    message::MsgDataBound,
};

use super::{COMPONENT_NAME, Error, InnerMessage, Row};

pub struct Input<TMsg, TFnInput>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(&TMsg) -> Result<Option<Vec<Row>>, Error> + Send + Sync,
{
    pub msgbus_input: MsgBusInput<TMsg>,
    pub output: mpsc::Sender<InnerMessage>,
    pub fn_input: TFnInput,
}

impl<TMsg, TFnInput> Input<TMsg, TFnInput>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(&TMsg) -> Result<Option<Vec<Row>>, Error> + Send + Sync,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let desc = format!("{COMPONENT_NAME} | task Input | channel output");

        while let Ok(msg) = self.msgbus_input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            let items = (self.fn_input)(&msg)?;
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
