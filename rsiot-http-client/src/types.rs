use crate::error::Error;

pub type Result_<T, TMessage> = Result<T, Error<TMessage>>;
