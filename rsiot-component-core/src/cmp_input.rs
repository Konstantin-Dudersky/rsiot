use tokio::sync::broadcast::{self, error::RecvError};
use uuid::Uuid;

use rsiot_messages_core::{
    message_v2::{Message, MsgSource},
    IMessage,
};

use crate::ComponentError;

#[derive(Debug)]
pub struct CmpInput<TMsg>
// where
//     TMsg: IMessage,
{
    channel: broadcast::Receiver<Message<TMsg>>,
    msg_source: MsgSource,
}

impl<TMsg> CmpInput<TMsg>
where
    TMsg: Clone,
{
    pub fn new(
        channel: broadcast::Receiver<Message<TMsg>>,
        executor_name: &str,
        executor_id: Uuid,
    ) -> Self {
        Self {
            channel,
            msg_source: MsgSource::new(executor_name, executor_id),
        }
    }

    pub(crate) fn set_component_id(&mut self, component_name: &str, component_id: Uuid) {
        self.msg_source.set_component(component_name, component_id);
    }

    pub(crate) fn set_session_id(&mut self, session_name: &str, session_id: Uuid) {
        self.msg_source.set_session(session_name, session_id);
    }

    pub async fn recv(&mut self) -> Result<Option<Message<TMsg>>, ComponentError> {
        let msg = self.channel.recv().await.map_err(|e| ComponentError::CmpInput(e.to_string()))?;
        // match msg.process {
        //     Some(msg_process) => todo!(),
        //     None => ,
        // }
        let msg_process = 

        if msg.process == self.msg_source {
            return Ok(None);
        }
        Ok(Some(msg))
    }
}

impl<TMsg> Clone for CmpInput<TMsg>
where
    TMsg: IMessage,
{
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.resubscribe(),
            executor_id: self.executor_id.clone(),
            component_id: self.component_id.clone(),
        }
    }
}
