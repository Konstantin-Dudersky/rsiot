use std::time::Duration;

use crate::message::Message;

pub struct Config<TMsg> {
    pub period: Duration,
    pub fn_output: fn() -> Vec<Message<TMsg>>,
}

pub struct SystemInfo {}
