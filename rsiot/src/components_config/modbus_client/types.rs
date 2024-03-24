use crate::message::Message;

use super::Response;

/// Коллбек при успешном опросе
pub type FnOnSuccess<TMessage> = fn(&Response) -> Vec<Message<TMessage>>;

/// Коллбек при неуспешном опросе
pub type FnOnFailure<TMessage> = fn() -> Vec<Message<TMessage>>;
