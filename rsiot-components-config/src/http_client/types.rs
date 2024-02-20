use anyhow::Result;
use rsiot_messages_core::message_v2::Message;

pub type CbkOnSuccess<TMsg> = fn(&str) -> Result<Vec<Message<TMsg>>>;
pub type CbkOnFailure<TMsg> = fn() -> Vec<Message<TMsg>>;
