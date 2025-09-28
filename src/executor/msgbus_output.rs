use tokio::sync::mpsc;
use tracing::trace;
use uuid::Uuid;

use crate::message::{Message, MsgDataBound};

use super::ComponentError;

/// Шина MsgBus - отправка исходящих сообщений
pub struct MsgBusOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    output: mpsc::Sender<Message<TMsg>>,
    id: Uuid,
}

impl<TMsg> MsgBusOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub(crate) fn new(output: mpsc::Sender<Message<TMsg>>, id: Uuid) -> Self {
        Self { output, id }
    }

    /// Отправка исходящих сообщений
    pub async fn send_output(&self, mut msg: Message<TMsg>) -> Result<(), ComponentError> {
        trace!("Start send to output: {msg:?}");

        msg.set_cmp_source(&self.id);
        self.output
            .send(msg)
            .await
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }

    /// Отправка исходящих сообщений, в синхронном окружении
    pub fn send_output_blocking(&self, mut msg: Message<TMsg>) -> Result<(), ComponentError> {
        trace!("Start send to output: {msg:?}");

        msg.set_cmp_source(&self.id);

        self.output
            .blocking_send(msg)
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }
}
