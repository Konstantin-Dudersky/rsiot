use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, CmpResult, Component, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "CMP_TEMPLATE";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg> IComponentProcess<Config<TMsg>, TMsg> for Component<Config<TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    async fn process(&self, config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> CmpResult {
        let in_out = msg_bus.clone_with_new_id(COMPONENT_NAME, AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

/// Компонент CMP_TEMPLATE
pub type Cmp<TMsg> = Component<Config<TMsg>, TMsg>;
