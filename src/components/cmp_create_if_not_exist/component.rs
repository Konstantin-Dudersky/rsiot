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
    TService: ServiceBound,
{
    async fn process(&self, config: Config<TMsg>, msg_bus: CmpInOut<TMsg, TService>) -> CmpResult {
        let in_out =
            msg_bus.clone_with_new_id("cmp_create_if_not_exist", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент cmp_create_if_not_exist
pub type Cmp<TMsg, TService> = Component<Config<TMsg>, TMsg, TService>;
