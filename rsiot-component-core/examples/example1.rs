//! Собрать такую цепочку:
//! 0 -> 1 -> 2 -> 3 ---> 9 -> 10
//!        -> 4 -> 5 --->
//!        -> 6 -> 7 -> 8
//!

use rsiot_component_core::{component_example, ComponentChain2 as ComponentChain};
use rsiot_messages_core::IMessage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
enum TestMessage {}

impl IMessage for TestMessage {}

fn main() {
    let _chain = ComponentChain::<TestMessage>::new(100)
        .add_cmp(component_example::new())
        .add_cmp(component_example::new())
        .split()
        .add_cmp(component_example::new())
        .add_cmp(component_example::new())
        .branch()
        .add_cmp(component_example::new())
        .add_cmp(component_example::new())
        .branch()
        .add_cmp(component_example::new())
        .add_cmp(component_example::new())
        .add_cmp(component_example::new())
        .join()
        .add_cmp(component_example::new())
        .add_cmp(component_example::new())
        .build();
}
