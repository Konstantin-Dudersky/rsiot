use async_trait::async_trait;

use rsiot_component_core::{Cache, CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::MsgDataBound;

use crate::fn_process::fn_process;

#[cfg_attr(feature = "single-thread", async_trait(?Send))]
#[cfg_attr(not(feature = "single-thread"), async_trait)]
impl<TMsg> IComponentProcess<crate::Config<TMsg>, TMsg> for Component<crate::Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: crate::Config<TMsg>,
        input: CmpInOut<TMsg>,
        _cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(input.clone_with_new_id("cmp_surrealdb"), config.clone()).await?;
        Ok(())
    }
}

pub type Cmp<TMsg> = Component<crate::Config<TMsg>, TMsg>;
