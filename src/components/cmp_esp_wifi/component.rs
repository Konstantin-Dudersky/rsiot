use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TService> IComponentProcess<Config, TMsg, TService> for Component<Config, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: Config,
        in_out: CmpInOut<TMsg, TService>,
    ) -> Result<(), ComponentError> {
        let in_out = in_out.clone_with_new_id("cmp_esp_wifi", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_wifi
pub type Cmp<TMsg, TService> = Component<Config, TMsg, TService>;
