use std::fmt::Debug;

use tokio::sync::mpsc;

use rsiot_messages_core::{Message, MsgDataBound, MsgSource};
use uuid::Uuid;

use crate::ComponentError;

#[derive(Clone, Debug)]
pub struct CmpOutput<TMsg> {
    channel: mpsc::Sender<Message<TMsg>>,
    msg_source: MsgSource,
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
    pub async fn send(&self, mut msg: Message<TMsg>) -> Result<(), ComponentError> {
        msg.cmp_set(&self.msg_source);
        self.channel
            .send(msg)
            .await
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }
}
