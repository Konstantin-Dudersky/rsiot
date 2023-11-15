use std::time::Duration;

use rsiot_messages_core::IMessage;

use crate::{
    request_param::RequestParam,
    types::{CbkOnFailure, CbkOnSuccess},
};

pub struct RequestPeriodic<TMessage>
where
    TMessage: IMessage,
{
    pub period: Duration,
    pub request_param: RequestParam,
    pub on_success: CbkOnSuccess<TMessage>,
    pub on_failure: CbkOnFailure<TMessage>,
}
