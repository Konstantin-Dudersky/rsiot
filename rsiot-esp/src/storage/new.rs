use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use rsiot_component_core::Component;
use rsiot_messages_core::IMessage;

use super::{config::Config, fn_process::fn_process};

pub fn new<TMessage, TConfig>(
    config: Config<TMessage, TConfig>,
) -> Box<Component<TMessage, Config<TMessage, TConfig>>>
where
    TMessage: IMessage + 'static,
    TConfig: Debug + Default + DeserializeOwned + PartialEq + Send + Serialize + 'static,
{
    Box::new(Component::new(config, fn_process))
}
