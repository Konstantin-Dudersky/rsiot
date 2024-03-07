//! cargo run -p rsiot --example create_signal_from_msg --features "cmp_leptos, single-thread"

#[cfg(feature = "cmp_leptos")]
fn main() {
    use rsiot::{
        components::cmp_leptos::create_signal_from_msg,
        message::{example_message::*, *},
    };

    // Раскомментировать для cargo-expand
    // use rsiot_macros::create_signal_from_msg;

    let (_signal, _signal_set) = create_signal_from_msg!("Custom-DataGroup-DataGroupF64");

    let (_signal, _signal_set) = create_signal_from_msg::create(create_signal_from_msg::Config {
        default: Message::new(MsgData::Custom(Custom::ValueInstantF64(Default::default()))),
        fn_input: |msg| {
            let value = &msg.data;
            match value {
                MsgData::Custom(Custom::ValueInstantF64(value)) => Some(*value),
                _ => None,
            }
        },
        fn_output: |value| {
            Some(Message::new(MsgData::Custom(Custom::ValueInstantF64(
                value,
            ))))
        },
    });
}

#[cfg(not(feature = "cmp_leptos"))]
fn main() {}
