#[cfg(target_arch = "wasm32")]
fn main() {
    use rsiot_leptos::create_signal_from_msg;
    use rsiot_messages_core::{example_message::*, *};

    // Раскомментировать для cargo-expand
    // use rsiot_macros::create_signal_from_msg;

    let (_signal, _signal_set) =
        create_signal_from_msg!("MsgType::Custom-Custom::DataGroup-DataGroup::DataGroupF64");

    let (_signal, _signal_set) = create_signal_from_msg::create(create_signal_from_msg::Config {
        default: Message::new_full(MsgType::Custom(Custom::ValueInstantF64(Default::default()))),
        fn_input: |msg| {
            let value = &msg.data;
            match value {
                MsgType::Custom(value) => match value {
                    Custom::ValueInstantF64(value) => Some(value.clone()),
                    _ => None,
                },
                _ => None,
            }
        },
        fn_output: |value| {
            Some(Message::new_full(MsgType::Custom(Custom::ValueInstantF64(
                value,
            ))))
        },
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {}
