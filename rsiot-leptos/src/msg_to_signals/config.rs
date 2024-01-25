use rsiot_messages_core::{IMessage, MsgContent};

pub struct Config<TMsg, TValue>
where
    TMsg: IMessage + 'static,
{
    /// Значение по-умолчанию
    ///
    /// Пример:
    /// ```rust
    /// Message::Variant(MsgContent::default())
    /// ```
    pub default: TMsg,

    /// Преобразование сообщения в сигнал чтения
    ///
    /// Пример:
    /// ```rust
    /// |msg| match msg {
    ///     Message::Variant(content) => Some(content.clone()),
    /// _ => None,
    /// }
    /// ```
    pub fn_input: fn(&TMsg) -> Option<MsgContent<TValue>>,

    /// Преоборазование сигнала записи в сообщение
    ///
    /// Если сигнал только для чтения:
    /// ```rust
    /// |_| None
    /// ```
    pub fn_output: fn(MsgContent<TValue>) -> Option<TMsg>,
}
