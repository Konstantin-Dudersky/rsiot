use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::fn_process::fn_process;

#[cfg_attr(feature = "single-thread", async_trait(?Send))]
#[cfg_attr(not(feature = "single-thread"), async_trait)]
impl<TMsg> IComponentProcess<super::Config<TMsg>, TMsg> for Component<super::Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(
        &self,
        config: super::Config<TMsg>,
        input: CmpInOut<TMsg>,
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
pub type Cmp<TMsg> = Component<super::Config<TMsg>, TMsg>;
