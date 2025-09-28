use tokio::sync::mpsc;

use crate::{
    components_config::http_client::{MsgResponse, RequestInput, RequestPeriodic},
    executor::MsgBusOutput,
    message::{Message, MsgDataBound},
};

use super::{Error, Result};

pub struct Response<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: mpsc::Receiver<MsgResponse>,
    pub output: MsgBusOutput<TMsg>,
    /// Запросы, которые формируются на основе входящих сообщений
    pub requests_input: Vec<Box<dyn RequestInput<TMsg>>>,
    /// Периодические запросы
    pub requests_periodic: Vec<Box<dyn RequestPeriodic<TMsg>>>,
}

impl<TMsg> Response<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Some(msg_response) = self.input.recv().await {
            let mut messages = vec![];
            for ric in self.requests_input.iter() {
                let msgs = ric.process_response(&msg_response);
                if let Some(msgs) = msgs {
                    messages.extend(msgs);
                }
            }

            for rpc in self.requests_periodic.iter() {
                let msgs = rpc.process_response(&msg_response);
                if let Some(msgs) = msgs {
                    messages.extend(msgs);
                }
            }

            for msg in messages {
                let msg = Message::new_custom(msg);
                self.output
                    .send(msg)
                    .await
                    .map_err(|_| Error::TokioSyncMpscSend)?;
            }
        }

        Err(Error::TaskProcessResponse)
    }
}
