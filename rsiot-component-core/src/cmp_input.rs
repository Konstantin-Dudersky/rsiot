use tokio::sync::broadcast::{self, error::RecvError};

use rsiot_messages_core::{
    msg_meta::{ComponentId, ExecutorId},
    IMessage,
};
use tracing::warn;

#[derive(Debug)]
pub struct CmpInput<TMsg>
where
    TMsg: IMessage,
{
    channel: broadcast::Receiver<TMsg>,
    executor_id: ExecutorId,
    component_id: Option<ComponentId>,
}

impl<TMsg> CmpInput<TMsg>
where
    TMsg: IMessage,
{
    pub fn new(channel: broadcast::Receiver<TMsg>, service_id: ExecutorId) -> Self {
        Self {
            channel,
            executor_id: service_id,
            component_id: None,
        }
    }

    pub(crate) fn set_component_name(&mut self, component_name: &str) -> ComponentId {
        let component_id = ComponentId::new(&self.executor_id, component_name);
        self.component_id = Some(component_id.clone());
        component_id
    }

    pub async fn recv(&mut self) -> Result<Option<TMsg>, RecvError> {
        let msg = self.channel.recv().await?;
        if msg.cmp_process() == self.component_id {
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
