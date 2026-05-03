use tokio::sync::mpsc;

use crate::{
    executor::{CheckCapacity, MsgBusInput},
    message::MsgDataBound,
};

use super::{COMPONENT_NAME, ConfigTable, Error, InnerMessage};

pub struct Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub msgbus_input: MsgBusInput<TMsg>,
    pub output: mpsc::Sender<InnerMessage>,
    pub table: ConfigTable<TMsg>,
}

impl<TMsg> Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let desc = format!("{COMPONENT_NAME} | task Input | channel output");

        while let Ok(msg) = self.msgbus_input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            let row = (self.table.fn_input)(&msg)?;

            let Some(row) = row else { continue };
            self.output
                .check_capacity(0.2, &desc)
                .send(InnerMessage::Row(row))
                .await
                .map_err(|_| Error::TokioMpsc { task_name: "Input" })?;
        }
        Err(Error::TaskInputEnd)
    }
}
