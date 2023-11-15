use std::time::Duration;

use rsiot_messages_core::IMessage;

use crate::request::Request;

type Callback<TMessage> = fn() -> Vec<TMessage>;

pub struct RequestCyclic<TMessage>
where
    TMessage: IMessage,
{
    pub cycle: Duration,
    pub request_params: Request,
    pub on_success: Callback<TMessage>,
}
