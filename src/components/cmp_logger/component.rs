use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};
use async_trait::async_trait;

use super::{Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_logger";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: 'static + MsgDataBound,
{
    async fn process(
        &self,
        config: Config<TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config,
            in_out.clone_with_new_id(COMPONENT_NAME, AuthPermissions::FullAccess),
        )
        .await?;
        Ok(())
    }
}

/// Компонент cmp_logger
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
