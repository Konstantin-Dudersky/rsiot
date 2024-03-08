use anyhow::Result;

use crate::message::Message;

pub type CbkOnSuccess<TMsg> = fn(&str) -> Result<Vec<Message<TMsg>>>;
pub type CbkOnFailure<TMsg> = fn() -> Vec<Message<TMsg>>;
