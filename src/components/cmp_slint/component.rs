use async_trait::async_trait;
use slint::ComponentHandle;

use crate::{
    executor::{Component, ComponentError, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{Config, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_slint";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMainWindow, TMsg> IComponentProcess<Config<TMsg, TMainWindow>, TMsg>
    for Component<Config<TMsg, TMainWindow>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    Self: Sync,
    TMainWindow: ComponentHandle + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TMainWindow>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Ok(())
    }
}

/// Компонент cmp_slint
pub type Cmp<TMsg, TMainWindow> = Component<Config<TMsg, TMainWindow>, TMsg>;
