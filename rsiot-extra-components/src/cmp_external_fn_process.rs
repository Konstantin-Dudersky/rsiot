use rsiot_component_core::{Component, IComponentFunction};
use rsiot_messages_core::IMessage;

pub fn new<TMessage, TConfig>(
    config: TConfig,
    fn_process: impl IComponentFunction<TMessage, TConfig> + 'static,
) -> Box<Component<TMessage, TConfig>>
where
    TMessage: IMessage + 'static,
{
    let cmp = Component::new(config, fn_process);
    Box::new(cmp)
}
