use esp_idf_svc::hal::gpio::{Output, OutputPin, PinDriver};
use rsiot_component_core::{ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

pub struct GpioOutputConfig<'a, TPin, TMessage>
where
    TPin: OutputPin,
    TMessage: IMessage,
{
    pub driver: PinDriver<'a, TPin, Output>,
    pub fn_input: fn(&TMessage) -> Option<bool>,
    pub is_low_triggered: bool,
}

pub async fn gpio_output<TPin, TMessage>(
    mut input: ComponentInput<TMessage>,
    _output: ComponentOutput<TMessage>,
    mut config: GpioOutputConfig<'static, TPin, TMessage>,
) where
    TPin: OutputPin,
    TMessage: IMessage,
{
    while let Ok(msg) = input.recv().await {
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
