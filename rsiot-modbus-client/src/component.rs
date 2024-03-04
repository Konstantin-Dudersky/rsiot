use async_trait::async_trait;

use rsiot_component_core::{CmpInOut, Component, ComponentError, IComponentProcess};
use rsiot_messages_core::{AuthPermissions, MsgDataBound};

use crate::{config::ConfigNewType, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMessage> IComponentProcess<ConfigNewType<TMessage>, TMessage>
    for Component<ConfigNewType<TMessage>, TMessage>
where
    TMessage: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: ConfigNewType<TMessage>,
        in_out: CmpInOut<TMessage>,
    ) -> Result<(), ComponentError> {
        fn_process(
            in_out.clone_with_new_id("cmp_modbus_client", AuthPermissions::FullAccess),
            config.0,
        )
        .await
    }
}

pub type Cmp<TMessage> = Component<ConfigNewType<TMessage>, TMessage>;
