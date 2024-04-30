use crate::message::{Message, MsgDataBound};

/// Режим работы пина
#[derive(Clone)]
pub enum PCF8575PinMode<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Пин отключен
    Disabled,

    /// Пин в режиме входа
    Input {
        /// Преобразование входных сигналов в исходящие сообщения
        fn_output: fn(bool) -> Option<Message<TMsg>>,
    },

    /// Пин в режиме выхода
    Output {
        /// Преобразование входящих сообщений в сигналы управления выходами
        fn_input: fn(Message<TMsg>) -> Option<bool>,
    },
}
