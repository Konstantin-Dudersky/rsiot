use async_trait::async_trait;

use rsiot_component_core::{
    Cache, CmpOutput, Component, ComponentError, ComponentInput, IComponentProcess,
};
use rsiot_messages_core::IMessage;

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<ConfigAlias<TMsg>, TMsg> for Component<ConfigAlias<TMsg>, TMsg>
where
    TMsg: IMessage + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMsg>,
        _input: ComponentInput<TMsg>,
        output: CmpOutput<TMsg>,
        cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        fn_process(output, config, cache).await
    }
}

pub type Cmp<TMsg> = Component<ConfigAlias<TMsg>, TMsg>;
