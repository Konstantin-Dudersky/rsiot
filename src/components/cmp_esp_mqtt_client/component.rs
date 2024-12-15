use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TService> IComponentProcess<Config<TMsg>, TMsg, TService>
    for Component<Config<TMsg>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    async fn process(&self, config: Config<TMsg>, in_out: CmpInOut<TMsg, TService>) -> CmpResult {
        let in_out = in_out.clone_with_new_id("cmp_esp_mqtt_client", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_mqtt_client
pub type Cmp<TMsg, TService> = Component<Config<TMsg>, TMsg, TService>;
