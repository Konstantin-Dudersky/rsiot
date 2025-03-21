use async_trait::async_trait;
use slint::ComponentHandle;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{fn_process::fn_process, Config};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMainWindow, TMsg, TService> IComponentProcess<Config<TMsg, TMainWindow>, TMsg, TService>
    for Component<Config<TMsg, TMainWindow>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    Self: Sync,
    TMainWindow: ComponentHandle + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TMainWindow>,
        input: CmpInOut<TMsg, TService>,
    ) -> Result<(), ComponentError> {
        let input = input.clone_with_new_id("cmp_slint", AuthPermissions::FullAccess);
        fn_process(config, input).await?;
        Ok(())
    }
}

/// Компонент cmp_slint
pub type Cmp<TMsg, TMainWindow, TService> = Component<Config<TMsg, TMainWindow>, TMsg, TService>;
