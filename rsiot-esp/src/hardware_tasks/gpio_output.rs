use esp_idf_svc::hal::gpio::{Output, OutputPin, PinDriver};
use rsiot_component_core::{CmpInput, CmpOutput};
use rsiot_messages_core::{Message, MsgDataBound};

pub struct GpioOutputConfig<'a, TPin, TMsg>
where
    TPin: OutputPin,
    TMsg: MsgDataBound,
{
    pub driver: PinDriver<'a, TPin, Output>,
    pub fn_input: fn(&Message<TMsg>) -> Option<bool>,
    pub is_low_triggered: bool,
}

pub async fn gpio_output<TPin, TMessage>(
    mut input: CmpInput<TMessage>,
    _output: CmpOutput<TMessage>,
    mut config: GpioOutputConfig<'static, TPin, TMessage>,
) where
    TPin: OutputPin,
    TMessage: MsgDataBound,
{
    while let Ok(msg) = input.recv().await {
        let Some(msg) = msg else {
            continue;
        };
        let level = (config.fn_input)(&msg);
        let level = match level {
            Some(val) => val,
            None => continue,
        };
        if config.is_low_triggered ^ level {
            config.driver.set_low().unwrap();
        } else {
            config.driver.set_high().unwrap();
        }
    }
}
