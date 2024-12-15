use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::*,
};

use super::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TService> IComponentProcess<Config<TMsg>, TMsg, TService>
    for Component<Config<TMsg>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        in_out: CmpInOut<TMsg, TService>,
    ) -> Result<(), ComponentError> {
        fn_process(
            in_out.clone_with_new_id("cmp_derive", AuthPermissions::FullAccess),
            config,
        )
        .await
        .map_err(|e| ComponentError::Execution(e.to_string()))
    }
}

/// Компонент cmp_derive
pub type Cmp<TMsg, TService> = Component<Config<TMsg>, TMsg, TService>;
