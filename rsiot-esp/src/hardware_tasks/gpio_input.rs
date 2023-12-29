use esp_idf_svc::hal::gpio::{Input, InputPin, Level, PinDriver};

use rsiot_component_core::{ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

pub async fn gpio_input<TPin, TMessage>(
    _input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    mut driver: PinDriver<'static, TPin, Input>,
    fn_output: fn(&bool) -> TMessage,
) where
    TPin: InputPin,
    TMessage: IMessage,
{
    loop {
        let level = driver.get_level();
        let level = gpio_level_to_bool(&level);
        let msg = (fn_output)(&level);
        output.send(msg).await.unwrap();
        driver.wait_for_any_edge().await.unwrap();
    }
}

fn gpio_level_to_bool(level: &Level) -> bool {
    match level {
        Level::Low => true,
        Level::High => false,
    }
}
