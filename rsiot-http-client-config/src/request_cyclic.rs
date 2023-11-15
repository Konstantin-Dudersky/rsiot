use std::time::Duration;

use rsiot_messages_core::IMessage;

use crate::request_param::RequestParam;

type CbkOnSuccess<TMessage> = fn(String) -> Vec<TMessage>;
type CbkOnFailure<TMessage> = fn() -> Vec<TMessage>;

pub struct RequestCyclic<TMessage>
where
    TMessage: IMessage,
{
    pub cycle: Duration,
    pub request_params: RequestParam,
    pub on_success: CbkOnSuccess<TMessage>,
    pub on_failure: CbkOnFailure<TMessage>,
}
