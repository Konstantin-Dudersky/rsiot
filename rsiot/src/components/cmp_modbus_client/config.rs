use std::ops::Deref;

pub use crate::components::components_config::modbus_client::*;

pub struct ConfigNewType<TMessage>(pub Config<TMessage>);

impl<TMessage> Deref for ConfigNewType<TMessage> {
    type Target = Config<TMessage>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<TMessage> From<Config<TMessage>> for ConfigNewType<TMessage> {
    fn from(value: Config<TMessage>) -> Self {
        Self(value)
    }
}
