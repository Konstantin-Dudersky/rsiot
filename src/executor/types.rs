use tokio::sync::{broadcast, mpsc};

use crate::message::*;

use super::ComponentError;

/// Тип возвращаемого значения функции `process` компонента
pub type CmpResult = Result<(), ComponentError>;
pub type CmpInput<TMsg> = broadcast::Receiver<Message<TMsg>>;
pub type CmpOutput<TMsg> = mpsc::Sender<Message<TMsg>>;

/// Функция фильтрации сообщений в зависимости от авторизации
pub(crate) type FnAuth<TMsg> = fn(Message<TMsg>, &AuthPermissions) -> Option<Message<TMsg>>;
