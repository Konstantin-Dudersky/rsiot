use esp_idf_svc::hal::gpio::{Input, InputPin, PinDriver};

use crate::message::{Message, MsgDataBound};

pub struct Config<TPin, TMsg>
where
    TMsg: MsgDataBound,
    TPin: InputPin,
{
    pub fn_output: fn(bool) -> Message<TMsg>,
    pub driver: PinDriver<'static, TPin, Input>,
}
