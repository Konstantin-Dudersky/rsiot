use crate::message::{Message, MsgDataBound};

#[derive(Clone)]
pub enum PCF8575PinMode<TMsg>
where
    TMsg: MsgDataBound,
{
    Input {
        fn_output: fn(bool) -> Option<Message<TMsg>>,
    },
    Output {
        fn_input: fn(Message<TMsg>) -> Option<bool>,
    },
}
