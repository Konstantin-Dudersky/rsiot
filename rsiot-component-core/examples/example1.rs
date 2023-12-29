use rsiot_component_core::{component_example, ComponentCollection};
use rsiot_messages_core::ExampleMessage;

fn main() {
    let _chain = ComponentCollection::<ExampleMessage>::new(100, vec![component_example::new()]);
}
