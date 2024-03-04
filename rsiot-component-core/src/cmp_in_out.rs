use std::fmt::Debug;

use tokio::sync::{broadcast, mpsc};
use tracing::info;
use uuid::Uuid;

use rsiot_messages_core::*;

use crate::ComponentError;

#[derive(Debug)]
pub struct CmpInOut<TMsg> {
    input: broadcast::Receiver<Message<TMsg>>,
    output: mpsc::Sender<Message<TMsg>>,
    name: String,
    id: Uuid,
    auth_perm: AuthPermissions,
}

impl<TMsg> CmpInOut<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn new(
        input: broadcast::Receiver<Message<TMsg>>,
        output: mpsc::Sender<Message<TMsg>>,
        name: &str,
        auth_perm: AuthPermissions,
    ) -> Self {
        let id = MsgTrace::generate_uuid();
        info!("Start: {}, id: {}", name, id);
        Self {
            input,
            output,
            id,
            name: name.into(),
            auth_perm,
        }
    }

    pub fn clone_with_new_id(self, name: &str, auth_perm: AuthPermissions) -> Self {
        let name = format!("{}::{}", self.name, name);
        let id = MsgTrace::generate_uuid();
        info!("Start: {}, id: {}", name, id);
        Self {
            name,
            id,
            auth_perm,
            ..self
        }
    }

    pub async fn recv_input(&mut self) -> Result<Option<Message<TMsg>>, ComponentError> {
        let msg = self
            .input
            .recv()
            .await
            .map_err(|e| ComponentError::CmpInput(e.to_string()))?;
        if msg.contains_trace_item(&self.id) {
            return Ok(None);
        }
        Ok(Some(msg))
    }

    pub async fn send_output(&self, mut msg: Message<TMsg>) -> Result<(), ComponentError> {
        msg.add_trace_item(&self.id, &self.name);
        self.output
            .send(msg)
            .await
            .map_err(|e| ComponentError::CmpOutput(e.to_string()))
    }
}

impl<TMsg> Clone for CmpInOut<TMsg>
where
    TMsg: Clone,
{
    fn clone(&self) -> Self {
        Self {
            input: self.input.resubscribe(),
            output: self.output.clone(),
            id: self.id,
            name: self.name.clone(),
            auth_perm: self.auth_perm.clone(),
        }
    }
}
