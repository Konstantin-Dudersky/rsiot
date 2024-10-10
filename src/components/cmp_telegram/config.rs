use crate::message::{Message, MsgDataBound};

/// Конфигурация cmp_telegram
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Токен бота
    ///
    /// Определяется при создании бота через BotFather
    pub bot_token: String,

    /// Идентификатор чата, в который бот будет отправлять сообщения. Определить идентификатор можно разными способами. Один из способов - через телеграм бот  [usinfbot](https://t.me/usinfbot). Нужно переслать сообщение из канала в данный бот, в ответе будет идентификатор канала.
    pub chat_id: i64,

    /// # Пример
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: fn(Message<TMsg>) -> Option<String>,
}
