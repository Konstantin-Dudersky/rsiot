use anyhow::Result;

pub type CbkOnSuccess<TMessage> = fn(&str) -> Result<Vec<TMessage>>;
pub type CbkOnFailure<TMessage> = fn() -> Vec<TMessage>;
