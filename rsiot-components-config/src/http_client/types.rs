use anyhow::Result;
use rsiot_messages_core::Message;

pub type CbkOnSuccess<TMsg> = fn(&str) -> Result<Vec<Message<TMsg>>>;
pub type CbkOnFailure<TMsg> = fn() -> Vec<Message<TMsg>>;
