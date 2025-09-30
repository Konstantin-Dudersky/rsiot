use rsiot::{
    components::cmp_filesystem::{BufferBound, CallFnOutputKind, Cmp, Config},
    serde_utils::SerdeAlgKind,
};
use serde::{Deserialize, Serialize};

use super::messages::*;

pub fn new() -> rsiot::executor::Component<Config<Msg, Buffer>, Msg> {
    let config_filesystem = Config::<Msg, Buffer> {
        filename: "examples/cmp_filesystem/test_file.json".into(),
        serde_alg: SerdeAlgKind::Json,
        call_fn_output_kind: CallFnOutputKind::Always,
        fn_input: |msg, buffer| match msg {
            Msg::InjPeriodic(InjPeriodic::Increase) => {
                let mut buffer = buffer.clone();
                buffer.counter = buffer.counter.wrapping_add(1);
                Some(buffer)
            }
            _ => None,
        },
        fn_output: |buffer| vec![Msg::Filesystem(Filesystem::Counter(buffer.counter))],
    };

    Cmp::new(config_filesystem)
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Buffer {
    pub counter: u64,
    pub internal_struct: InternalStruct,
}

impl BufferBound for Buffer {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InternalStruct {
    pub field: f64,
}
