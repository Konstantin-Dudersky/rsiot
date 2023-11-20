use serde::{Deserialize, Serialize};

use rsiot_messages_core::IMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {}

fn main() {}
