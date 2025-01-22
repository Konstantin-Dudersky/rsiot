use async_trait::async_trait;
use leptos::*;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{fn_process::fn_process, Config, StoreBound};

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg, TView, TIntoView, TService, TInputStore, TOutputStore>
    IComponentProcess<Config<TMsg, TView, TIntoView, TInputStore, TOutputStore>, TMsg, TService>
    for Component<Config<TMsg, TView, TIntoView, TInputStore, TOutputStore>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TView: Fn() -> TIntoView + 'static,
    TIntoView: IntoView,
    TService: ServiceBound + 'static,
    TInputStore: StoreBound + 'static,
    TOutputStore: StoreBound + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TView, TIntoView, TInputStore, TOutputStore>,
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
pub type Cmp<TMsg, TView, TIntoView, TService, TInputStore, TOutputStore> =
    Component<Config<TMsg, TView, TIntoView, TInputStore, TOutputStore>, TMsg, TService>;
