use async_trait::async_trait;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{fn_process::fn_process, Config, StoreBound};

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg, TService, TInputStore, TOutputStore>
    IComponentProcess<Config<TMsg, TInputStore, TOutputStore>, TMsg, TService>
    for Component<Config<TMsg, TInputStore, TOutputStore>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TInputStore: StoreBound + 'static,
    TOutputStore: StoreBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TInputStore, TOutputStore>,
        input: CmpInOut<TMsg, TService>,
    ) -> Result<(), ComponentError> {
        fn_process(
            config,
            input.clone_with_new_id("cmp_leptos", AuthPermissions::FullAccess),
        )
        .await?;
        Err(ComponentError::Execution("Stop execution".into()))
    }
}

/// Компонент cmp_leptos
pub type Cmp<TMsg, TService, TInputStore, TOutputStore> =
    Component<Config<TMsg, TInputStore, TOutputStore>, TMsg, TService>;
