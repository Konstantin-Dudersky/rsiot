use std::fmt::Debug;

use tokio::sync::mpsc;
use uuid::Uuid;

use rsiot_messages_core::*;

use crate::ComponentError;

#[derive(Clone, Debug)]
pub struct CmpOutput<TMsg> {
    channel: mpsc::Sender<Message<TMsg>>,
    id: Uuid,
    name: String,
}

impl<TMsg> CmpOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn new(
        channel: mpsc::Sender<Message<TMsg>>,
        executor_name: &str,
        executor_id: Uuid,
    ) -> Self {
        Self {
            channel,
            id: executor_id,
            name: executor_name.into(),
        }
    }

    pub(crate) fn set_component_id(&mut self, name: &str, id: Uuid) {
        self.id = id;
        self.name = format!("{}::{}", self.name, name);
    }

    pub(crate) fn set_session_id(&mut self, name: &str, id: Uuid) {
        self.id = id;
        self.name = format!("{}::{}", self.name, name);
    }
    pub async fn send(&self, mut msg: Message<TMsg>) -> Result<(), ComponentError> {
        msg.add_trace_item(&self.id, &self.name);
        self.channel
            .send(msg)
            .await
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }
}
