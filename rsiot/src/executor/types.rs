use tokio::sync::{broadcast, mpsc};

use rsiot_messages_core::*;

use super::ComponentError;

pub type ComponentResult = Result<(), ComponentError>;
pub(crate) type CmpInput<TMsg> = broadcast::Receiver<Message<TMsg>>;
pub(crate) type CmpOutput<TMsg> = mpsc::Sender<Message<TMsg>>;

/// Функция фильтрации сообщений в зависимости от авторизации
pub(crate) type FnAuth<TMsg> = fn(Message<TMsg>, &AuthPermissions) -> Option<Message<TMsg>>;
