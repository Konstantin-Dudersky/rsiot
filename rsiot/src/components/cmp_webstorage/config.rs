use rsiot_messages_core::*;

pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Вид хранилища - localStorage или sessionStorage
    pub kind: ConfigKind,

    pub fn_input: fn(Message<TMsg>) -> Option<Message<TMsg>>,

    pub fn_output: fn(Message<TMsg>) -> Option<Message<TMsg>>,
}

/// Вид хранилища
pub enum ConfigKind {
    LocalStorage,
    SessionStorage,
}
