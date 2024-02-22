use async_trait::async_trait;

use rsiot_component_core::{
    cmp_set_component_id, Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::MsgDataBound;

use crate::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        cmp_set_component_id(&mut input, &mut output, "cmp_auth");
        fn_process(input, output, config, cache).await
    }
}

pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
