use async_trait::async_trait;
use slint::ComponentHandle;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMainWindow, TMsg> IComponentProcess<Config<TMainWindow, TMsg>, TMsg>
    for Component<Config<TMainWindow, TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    Self: Sync,
    TMainWindow: ComponentHandle + 'static,
{
    async fn process(
        &self,
        config: Config<TMainWindow, TMsg>,
        input: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let input = input.clone_with_new_id("cmp_slint", AuthPermissions::FullAccess);
        fn_process(config, input).await?;
        Ok(())
    }
}

/// Компонент cmp_slint
pub type Cmp<TMainWindow, TMsg> = Component<Config<TMainWindow, TMsg>, TMsg>;
