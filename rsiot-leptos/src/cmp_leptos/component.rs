use async_trait::async_trait;
use leptos::*;
use tracing::info;

use rsiot_component_core::{
    cmp_set_component_name, Cache, CmpInput, CmpOutput, Component, ComponentError,
    IComponentProcess,
};
use rsiot_messages_core::MsgDataBound;

use super::{fn_process::fn_process, Config};

#[cfg(feature = "single-thread")]
#[async_trait(?Send)]
impl<TMsg, TView, TIntoView> IComponentProcess<Config<TView, TIntoView>, TMsg>
    for Component<Config<TView, TIntoView>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TView: Fn() -> TIntoView + 'static,
    TIntoView: IntoView,
{
    async fn process(
        &self,
        config: Config<TView, TIntoView>,
        mut input: CmpInput<TMsg>,
        mut output: CmpOutput<TMsg>,
        cache: Cache<TMsg>,
    ) -> Result<(), ComponentError> {
        info!("Starting cmp_leptos");
        cmp_set_component_name(&mut input, &mut output, "cmp_leptos");
        fn_process(config, input, output, cache).await?;
        Err(ComponentError::Execution("Stop execution".into()))
    }
}

pub type Cmp<TMsg, TView, TIntoView> = Component<Config<TView, TIntoView>, TMsg>;
