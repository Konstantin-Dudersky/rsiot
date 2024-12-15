use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::fn_process::fn_process;

#[cfg_attr(feature = "single-thread", async_trait(?Send))]
#[cfg_attr(not(feature = "single-thread"), async_trait)]
impl<TMsg, TService> IComponentProcess<super::Config<TMsg>, TMsg, TService>
    for Component<super::Config<TMsg>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: super::Config<TMsg>,
        input: CmpInOut<TMsg, TService>,
    ) -> Result<(), ComponentError> {
        fn_process(
            input.clone_with_new_id("cmp_surrealdb", AuthPermissions::FullAccess),
            config.clone(),
        )
        .await?;
        Ok(())
    }
}

/// Компонент cmp_surrealdb
pub type Cmp<TMsg, TService> = Component<super::Config<TMsg>, TMsg, TService>;
