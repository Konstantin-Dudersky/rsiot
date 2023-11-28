pub type CbkOnSuccess<TMessage> = fn(&str) -> Vec<TMessage>;
pub type CbkOnFailure<TMessage> = fn() -> Vec<TMessage>;
