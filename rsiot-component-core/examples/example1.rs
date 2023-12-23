use rsiot_component_core::{component_example, ComponentChain};
use rsiot_messages_core::ExampleMessage;

fn main() {
    let _chain = ComponentChain::<ExampleMessage>::new(100, vec![component_example::new()]);
}
