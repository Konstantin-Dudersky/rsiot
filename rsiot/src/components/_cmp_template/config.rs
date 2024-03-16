use crate::message::{Message, MsgDataBound};

#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn_input: fn(Message<TMsg>) -> Option<String>,
    pub fn_output: fn(String) -> Vec<Message<TMsg>>,
}
