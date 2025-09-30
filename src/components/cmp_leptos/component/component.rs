use async_trait::async_trait;

use crate::{
    executor::{Component, ComponentError, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{Config, StoreBound, fn_process::fn_process};

/// Название компонента
pub const COMPONENT_NAME: &str = "cmp_leptos";

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg, TInputStore, TOutputStore>
    IComponentProcess<Config<TMsg, TInputStore, TOutputStore>, TMsg>
    for Component<Config<TMsg, TInputStore, TOutputStore>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TInputStore: StoreBound + 'static,
    TOutputStore: StoreBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TInputStore, TOutputStore>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> Result<(), ComponentError> {
        fn_process(config, msgbus_linker.init(COMPONENT_NAME)).await?;
        Err(ComponentError::Execution("Stop execution".into()))
    }
}

/// Компонент cmp_leptos
pub type Cmp<TMsg, TInputStore, TOutputStore> =
    Component<Config<TMsg, TInputStore, TOutputStore>, TMsg>;
