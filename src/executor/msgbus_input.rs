use tokio::sync::broadcast;
use tracing::warn;
use uuid::Uuid;

use crate::message::{Message, MsgDataBound};

use super::ComponentError;

/// Шина MsgBus - получение входящих сообщений
pub struct MsgBusInput<TMsg>
where
    TMsg: MsgDataBound,
{
    input: broadcast::Receiver<Message<TMsg>>,
    name: String,
    id: Uuid,
}

impl<TMsg> MsgBusInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub(crate) fn new(input: broadcast::Receiver<Message<TMsg>>, name: &str, id: Uuid) -> Self {
        Self {
            input,
            name: name.to_string(),
            id,
        }
    }

    /// Получение входящих сообщений
    pub async fn recv(&mut self) -> Result<Message<TMsg>, ComponentError> {
        loop {
            let msg = self.input.recv().await;

            let msg = match msg {
                Ok(v) => v,
                Err(err) => {
                    warn!(
                        "MsgBusInput.recv_input() of component {} input error: {}",
                        self.name, err
                    );
                    continue;
                }
            };

            // Если данное сообщение было сгенерировано данным сервисом, пропускаем
            if msg.check_source(&self.id) {
                continue;
            }

            return Ok(msg);
        }
    }
}

impl<TMsg> Clone for MsgBusInput<TMsg>
where
    TMsg: MsgDataBound,
{
    fn clone(&self) -> Self {
        Self {
            input: self.input.resubscribe(),
            name: self.name.clone(),
            id: self.id,
        }
    }
}
