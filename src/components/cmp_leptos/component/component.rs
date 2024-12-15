use async_trait::async_trait;
use leptos::*;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound, ServiceBound},
};

use super::{fn_process::fn_process, Config};

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg, TView, TIntoView, TService> IComponentProcess<Config<TView, TIntoView>, TMsg, TService>
    for Component<Config<TView, TIntoView>, TMsg, TService>
where
    TMsg: MsgDataBound + 'static,
    TView: Fn() -> TIntoView + 'static,
    TIntoView: IntoView,
    TService: ServiceBound + 'static,
{
    async fn process(
        &self,
        config: Config<TView, TIntoView>,
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
pub type Cmp<TMsg, TView, TIntoView, TService> =
    Component<Config<TView, TIntoView>, TMsg, TService>;
