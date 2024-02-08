#![cfg(target_arch = "wasm32")]

use rsiot_leptos::create_signal_from_msg;
use rsiot_messages_core::{ExampleMessage, MsgContent};

fn main() {
    let (_signal, _signal_set) = create_signal_from_msg!("ExampleMessage::ValueInstantF64");
}
