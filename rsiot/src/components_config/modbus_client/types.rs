use crate::message::Message;

use super::Response;

pub type FnOnSuccess<TMessage> = fn(&Response) -> Vec<Message<TMessage>>;
pub type FnOnFailure<TMessage> = fn() -> Vec<Message<TMessage>>;