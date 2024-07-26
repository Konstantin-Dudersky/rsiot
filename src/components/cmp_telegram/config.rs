use crate::message::{Message, MsgDataBound};

/// Конфигурация cmp_telegram
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Токен бота
    pub bot_token: String,

    /// Идентфикатор пользователя или канала, в который рассылаются сообщения
    pub chat_id: i64,

    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: fn(Message<TMsg>) -> Option<String>,
}
