use crate::message::*;

/// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#Config
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#kind
    pub kind: ConfigKind,

    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#fn_input
    pub fn_input: fn(Message<TMsg>) -> Option<Message<TMsg>>,

    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#fn_output
    pub fn_output: fn(Message<TMsg>) -> Option<Message<TMsg>>,
}

/// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#ConfigKind
pub enum ConfigKind {
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#LocalStorage
    LocalStorage,
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#SessionStorage
    SessionStorage,
}

#[cfg(test)]
mod tests {
    use crate::{components::cmp_webstorage, message::example_message::*};

    #[test]
    #[allow(clippy::no_effect)]
    fn fn_input() {
        cmp_webstorage::Config::<Custom> {
            kind: cmp_webstorage::ConfigKind::SessionStorage,
            // ANCHOR: fn_input_save_all
            fn_input: Some,
            // ANCHOR_END: fn_input_save_all
            // ANCHOR: fn_output_not_load
            fn_output: |_| None,
            // ANCHOR_END: fn_output_not_load
        };
    }
}
