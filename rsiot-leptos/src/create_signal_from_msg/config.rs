use rsiot_messages_core::{Message, MsgDataBound};

pub struct Config<TMsg, TValue>
where
    TMsg: MsgDataBound + 'static,
    TValue: Default,
{
    /// Значение по-умолчанию
    ///
    /// Пример:
    /// ```rust
    /// Message::Variant(MsgContent::default())
    /// ```
    pub default: Message<TMsg>,

    /// Преобразование сообщения в сигнал чтения
    ///
    /// Пример:
    /// ```rust
    /// |msg| match msg {
    ///     Message::Variant(content) => Some(content.clone()),
    /// _ => None,
    /// }
    /// ```
    pub fn_input: fn(&Message<TMsg>) -> Option<TValue>,

    /// Преоборазование сигнала записи в сообщение
    ///
    /// Если сигнал только для чтения:
    /// ```rust
    /// |_| None
    /// ```
    pub fn_output: fn(TValue) -> Option<Message<TMsg>>,
}
