use super::Response;

pub type FnOnSuccess<TMessage> = fn(&Response) -> Vec<TMessage>;
pub type FnOnFailure<TMessage> = fn() -> Vec<TMessage>;
