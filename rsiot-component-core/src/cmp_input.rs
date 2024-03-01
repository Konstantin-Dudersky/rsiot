use std::fmt::Debug;

use tokio::sync::broadcast::{self};
use uuid::Uuid;

use rsiot_messages_core::*;

use crate::ComponentError;

#[derive(Debug)]
pub struct CmpInput<TMsg> {
    channel: broadcast::Receiver<Message<TMsg>>,
    id: Uuid,
    name: String,
}

impl<TMsg> CmpInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn new(
        channel: broadcast::Receiver<Message<TMsg>>,
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

    pub async fn recv(&mut self) -> Result<Option<Message<TMsg>>, ComponentError> {
        let msg = self
            .channel
            .recv()
            .await
            .map_err(|e| ComponentError::CmpInput(e.to_string()))?;
        if msg.contains_trace_item(&self.id) {
            return Ok(None);
        }
        Ok(Some(msg))
    }
}

impl<TMsg> Clone for CmpInput<TMsg>
where
    TMsg: Clone,
{
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.resubscribe(),
            id: self.id,
            name: self.name.clone(),
        }
    }
}
