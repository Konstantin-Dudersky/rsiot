use async_trait::async_trait;

use rsiot_component_core::{CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::{AuthPermissions, MsgDataBound};

use crate::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config,
            in_out.clone_with_new_id("cmp_webstorage", AuthPermissions::FullAccess),
        )
        .await
        .map_err(|err| ComponentError::Execution(err.to_string()))
    }
}

pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;