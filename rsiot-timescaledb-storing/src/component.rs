use async_trait::async_trait;

use rsiot_component_core::{
    cmp_set_component_id, Cache, CmpInput, CmpOutput, Component, ComponentError, IComponentProcess,
};
use rsiot_messages_core::MsgDataBound;

use crate::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<ConfigAlias, TMsg> for Component<ConfigAlias, TMsg>
where
    TMsg: MsgDataBound,
{
    async fn process(
        &self,
        config: ConfigAlias,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        cmp_set_component_id(&mut input, &mut output, "cmp_timescaledb_storing");
        fn_process(input, config).await
    }
}

pub type Cmp<TMsg> = Component<ConfigAlias, TMsg>;
