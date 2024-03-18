use esp_idf_svc::hal::gpio::{Output, OutputPin, PinDriver};

use crate::message::{Message, MsgDataBound};

pub struct Config<TPin, TMsg>
where
    TMsg: MsgDataBound,
    TPin: OutputPin,
{
    /// Функция преобразования входящих сообщений в сигналы управления
    ///
    /// ```rust
    /// fn_input: |_| None
    /// ```
    pub fn_input: fn(Message<TMsg>) -> Option<bool>,

    pub driver: PinDriver<'static, TPin, Output>,

    pub is_low_triggered: bool,
}
