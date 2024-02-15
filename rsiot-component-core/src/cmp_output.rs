use tokio::sync::mpsc;

use rsiot_messages_core::{
    msg_meta::{ComponentId, ExecutorId},
    IMessage,
};

use crate::ComponentError;

#[derive(Clone, Debug)]
pub struct CmpOutput<TMsg>
where
    TMsg: IMessage,
{
    channel: mpsc::Sender<TMsg>,
    _executor_id: ExecutorId,
    component_id: Option<ComponentId>,
}

impl<TMsg> CmpOutput<TMsg>
where
    TMsg: IMessage,
{
    pub fn new(channel: mpsc::Sender<TMsg>, service_id: ExecutorId) -> Self {
        Self {
            channel,
            _executor_id: service_id,
            component_id: None,
        }
    }

    pub(crate) fn set_component_id(&mut self, component_name: ComponentId) {
        self.component_id = Some(component_name);
    }

    pub async fn send(&self, mut msg: TMsg) -> Result<(), ComponentError> {
        match &self.component_id {
            Some(val) => msg.cmp_set(val),
            None => {
                let err = "component_id not set, check component code";
                return Err(ComponentError::CmpOutput(err.into()));
            }
        }
        self.channel
            .send(msg)
            .await
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }
}
