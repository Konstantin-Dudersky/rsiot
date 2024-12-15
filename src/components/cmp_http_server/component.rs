use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{config::ConfigAlias, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TService> IComponentProcess<ConfigAlias<TMsg>, TMsg, TService>
    for Component<ConfigAlias<TMsg>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: ConfigAlias<TMsg>,
        in_out: CmpInOut<TMsg, TService>,
    ) -> Result<(), ComponentError> {
        let config = config.0;
        let in_out = in_out.clone_with_new_id("cmp_http_server", AuthPermissions::FullAccess);
        fn_process(in_out, config).await
    }
}

/// Компонент cmp_http_server
pub type Cmp<TMsg, TService> = Component<ConfigAlias<TMsg>, TMsg, TService>;
