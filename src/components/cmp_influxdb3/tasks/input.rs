use tokio::sync::mpsc;

use crate::{components_config::influxdb3::FnInput, executor::MsgBusInput, message::MsgDataBound};

use super::{Error, Result, send_to_database_message::SendToDatabaseMessage};

pub struct Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub output: mpsc::Sender<SendToDatabaseMessage>,
    pub fn_input: FnInput<TMsg>,
}

impl<TMsg> Input<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Ok(msg) = self.input.recv().await {
            let items = (self.fn_input)(&msg);
            let Some(items) = items else { continue };
            for item in items {
                self.output
                    .send(SendToDatabaseMessage::LineProtocolItem(item))
                    .await
                    .map_err(|_| Error::TokioMpsc)?;
            }
        }
        Err(Error::TaskInputEnd)
    }
}
