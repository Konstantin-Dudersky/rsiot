use crate::IMessageChannel;

#[derive(Clone, Debug)]
pub enum ExampleMessageChannel {
    Output,
}

impl IMessageChannel for ExampleMessageChannel {}
