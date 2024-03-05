use rsiot_messages_core::*;

pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn_input: fn(&Message<TMsg>) -> anyhow::Result<Option<(String, String)>>,
    pub fn_output: fn(&str) -> anyhow::Result<Option<Message<TMsg>>>,
}
