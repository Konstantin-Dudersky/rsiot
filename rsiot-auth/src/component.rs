use async_trait::async_trait;

use rsiot_component_core::{
    cmp_set_component_name, Cache, CmpInput, CmpOutput, Component, ComponentError, ComponentResult,
    IComponentProcess,
};
use rsiot_messages_core::MsgDataBound;

use crate::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config, TMsg> for Component<Config, TMsg>
where
    TMsg: MsgDataBound,
{
    async fn process(
        &self,
        config: Config,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        cache: Cache<TMsg>,
    ) -> ComponentResult {
        cmp_set_component_name(&mut input, &mut output, "cmp_auth");
        fn_process(input, output, config, cache)
            .await
            .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

pub type Cmp<TMsg> = Component<Config, TMsg>;
