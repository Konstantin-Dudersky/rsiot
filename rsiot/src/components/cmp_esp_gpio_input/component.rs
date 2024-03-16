use async_trait::async_trait;
use esp_idf_svc::hal::gpio::InputPin;

use crate::{
    executor::{CmpInOut, Component, ComponentError, IComponentProcess},
    message::{AuthPermissions, MsgDataBound},
};

use super::{config::Config, fn_process::fn_process};

#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
impl<TPin, TMsg> IComponentProcess<Config<TPin, TMsg>, TMsg> for Component<Config<TPin, TMsg>, TMsg>
where
    TMsg: MsgDataBound + 'static,
    TPin: InputPin,
{
    async fn process(
        &self,
        config: Config<TPin, TMsg>,
        in_out: CmpInOut<TMsg>,
    ) -> Result<(), ComponentError> {
        let in_out = in_out.clone_with_new_id("cmp_esp_gpio_input", AuthPermissions::FullAccess);
        fn_process(config, in_out).await?;
        Ok(())
    }
}

pub type Cmp<TPin, TMsg> = Component<Config<TPin, TMsg>, TMsg>;
