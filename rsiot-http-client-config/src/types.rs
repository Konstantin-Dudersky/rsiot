pub type CbkOnSuccess<TMessage> = fn(String) -> Vec<TMessage>;
pub type CbkOnFailure<TMessage> = fn() -> Vec<TMessage>;
