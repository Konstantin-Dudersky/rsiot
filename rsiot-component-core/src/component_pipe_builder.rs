use tokio::sync::mpsc;

use rsiot_messages_core::IMessage;

use crate::{
    icomponent::IComponent,
    types::{StreamInput, StreamOutput},
};

struct ComponentPipe<TMessage> {
    components: Vec<Box<dyn IComponent<TMessage>>>,
}

pub struct ComponentPipeBuilder<TMessage> {
    buffer: usize,
    components: Vec<Box<dyn IComponent<TMessage>>>,
    next_receive: Option<StreamInput<TMessage>>,
}

impl<TMessage> ComponentPipeBuilder<TMessage> {
    pub fn new(buffer: usize) -> Self {
        Self {
            buffer,
            components: vec![],
            next_receive: None,
        }
    }

    pub fn begin(
        mut self,
        mut component: Box<dyn IComponent<TMessage>>,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<TMessage>(self.buffer);

        component.set_stream_output(tx);
        self.next_receive = Some(rx);
        self.components.push(component);
        self
    }

    pub fn end(mut self, mut component: Box<dyn IComponent<TMessage>>) -> () {
        let rx = match self.next_receive.take() {
            Some(val) => val,
            None => panic!("sadfsdf"),
        };

        component.set_stream_input(rx);
        self.components.push(component);
        for comp in self.components.iter_mut() {
            comp.spawn();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {}
}
