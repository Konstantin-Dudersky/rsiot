use crate::message::{Message, MsgDataBound};

#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: Vec<ConfigInput<TMsg>>,

    /// # Пример
    ///
    /// ```rust
    /// fn_output: |_| vec![]
    /// ```
    pub fn_output: fn(String) -> Vec<Message<TMsg>>,
}

#[derive(Clone)]
pub struct ConfigInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub filename: String,
    pub fn_save: fn(Message<TMsg>) -> Option<String>,
}
