use std::fmt::Debug;

use tokio::sync::broadcast::{self};
use uuid::Uuid;

use rsiot_messages_core::message_v2::{Message, MsgSource};

use crate::ComponentError;

#[derive(Debug)]
pub struct CmpInput<TMsg> {
    channel: broadcast::Receiver<Message<TMsg>>,
    msg_source: MsgSource,
}

impl<TMsg> CmpInput<TMsg>
where
    TMsg: Clone + Debug,
{
    pub fn new(
        channel: broadcast::Receiver<Message<TMsg>>,
        executor_name: &str,
        executor_id: Uuid,
    ) -> Self {
        let msg_source = MsgSource::new(executor_name, executor_id);
        Self {
            channel,
            msg_source,
        }
    }

    pub(crate) fn set_component_id(&mut self, component_name: &str, component_id: Uuid) {
        self.msg_source.set_component(component_name, component_id);
    }

    pub(crate) fn set_session_id(&mut self, session_name: &str, session_id: Uuid) {
        self.msg_source.set_session(session_name, session_id);
    }

    pub async fn recv(&mut self) -> Result<Option<Message<TMsg>>, ComponentError> {
        let msg = self
            .channel
            .recv()
            .await
            .map_err(|e| ComponentError::CmpInput(e.to_string()))?;
        let msg_cmp_process = match &msg.process {
            Some(cmp_process) => cmp_process,
            None => {
                let err = format!("cmp_process not set for message: {:?}", msg);
                return Err(ComponentError::CmpInput(err));
            }
        };
        if msg_cmp_process == &self.msg_source {
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
            msg_source: self.msg_source.clone(),
        }
    }
}
