use async_trait::async_trait;
use esp_idf_svc::hal::rmt::RmtChannel;

use crate::{
    executor::{CmpResult, Component, IComponentProcess, MsgBusLinker},
    message::MsgDataBound,
};

use super::{config::Config, fn_process::fn_process};

pub const COMPONENT_NAME: &str = "cmp_esp_led";

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TMsg, TRmt> IComponentProcess<Config<TMsg, TRmt>, TMsg> for Component<Config<TMsg, TRmt>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TRmt: RmtChannel + 'static,
{
    async fn process(
        &self,
        config: Config<TMsg, TRmt>,
        msgbus_linker: MsgBusLinker<TMsg>,
    ) -> CmpResult {
        let input = msgbus_linker.init(COMPONENT_NAME).input();
        fn_process(config, input).await?;
        Ok(())
    }
}

/// Компонент cmp_esp_led
pub type Cmp<TMsg, TRmt> = Component<Config<TMsg, TRmt>, TMsg>;
