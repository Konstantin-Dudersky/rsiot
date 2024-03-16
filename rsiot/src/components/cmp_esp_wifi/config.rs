use esp_idf_svc::{eventloop::EspSystemEventLoop, wifi::EspWifi};

use crate::message::{Message, MsgDataBound};

pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    pub fn_input: fn(Message<TMsg>) -> Option<String>,
    pub fn_output: fn(String) -> Vec<Message<TMsg>>,
    pub driver: EspWifi<'static>,
    pub event_loop: EspSystemEventLoop,
}
