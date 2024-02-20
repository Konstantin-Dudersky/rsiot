use async_trait::async_trait;

use rsiot_component_core::{
    cmp_set_component_id, Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<ConfigAlias<TMsg>, TMsg> for Component<ConfigAlias<TMsg>, TMsg>
where
    TMsg: 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMsg>,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        cmp_set_component_id(&mut input, &mut output, "cmp_http_client");
        fn_process(input, output, config).await
    }
}

pub type Cmp<TMsg> = Component<ConfigAlias<TMsg>, TMsg>;
