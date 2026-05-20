use rsiot::components::cmp_derive::*;

use crate::messages::*;

pub fn cmp() -> Cmp<Msg, Buffer> {
    let config = Config {
        output_send: ConfigOutputSend::OnEveryChange,
        fn_input: |msg, buffer| {
            let buffer = match msg {
                Msg::Counter1(v) => Buffer {
                    counter1: *v,
                    ..*buffer
                },
                Msg::Counter2(v) => Buffer {
                    counter2: *v,
                    ..*buffer
                },
                _ => return None,
            };
            Some(buffer)
        },
        fn_output: |buffer| Msg::DeriveMessage {
            counter1: buffer.counter1,
            counter2: buffer.counter2,
        },
    };

    Cmp::new(config)
}

#[derive(Clone, Default)]
pub struct Buffer {
    counter1: u32,
    counter2: u32,
}

impl BufferBound for Buffer {}
