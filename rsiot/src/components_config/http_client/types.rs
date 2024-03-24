use anyhow::Result;

use crate::message::Message;

/// Коллбек успешного ответа
pub type CbkOnSuccess<TMsg> = fn(&str) -> Result<Vec<Message<TMsg>>>;

/// Коллбек неуспешного ответа
pub type CbkOnFailure<TMsg> = fn() -> Vec<Message<TMsg>>;
