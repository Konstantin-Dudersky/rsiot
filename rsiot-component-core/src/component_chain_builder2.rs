use tokio::sync::mpsc;

use rsiot_messages_core::IMessage;

use crate::{ComponentChain, IComponent, StreamInput};

/// Построитель для цепочки компонентов
pub struct ComponentChainBuilder<TMessage> {
    /// Размер буфера сообщений в канале
    buffer: usize,
    /// Коллекция компонентов
    components: Vec<Box<dyn IComponent<TMessage>>>,
    /// Используется для сохраниния приемной стороны канала сообщений
    next_rx: StreamInput<TMessage>,
    nodes: Vec<Node>,
    mode: States,
    split_node: Option<Node>,
    end_branch_nodes: Vec<Node>,
}

impl<TMessage> ComponentChainBuilder<TMessage>
where
    TMessage: IMessage,
{
    pub fn new(buffer: usize) -> Self {
        Self {
            buffer,
            components: vec![],
            next_rx: None,
            nodes: vec![],
            mode: States::Start,
            split_node: None,
            end_branch_nodes: vec![],
        }
    }

    /// Начинаем цепочку
    pub fn start_cmp(mut self, mut component: Box<dyn IComponent<TMessage>>) -> Self {
        let (tx, rx) = mpsc::channel::<TMessage>(self.buffer);
        component.set_output(Some(tx));
        self.next_rx = Some(rx);
        self.components.push(component);
        self
    }

    /// Продолжаем цепочку
    pub fn then_cmp(mut self, mut component: Box<dyn IComponent<TMessage>>) -> Self {
        let (tx, rx) = mpsc::channel::<TMessage>(self.buffer);
        let next_rx = self.next_rx.take();
        component.set_input(next_rx);
        component.set_output(Some(tx));
        self.components.push(component);
        self.next_rx = Some(rx);
        self
    }

    /// Заканчиваем цепочку, возращаем готовую коллекцию компонентов
    pub fn end_cmp(
        mut self,
        mut component: Box<dyn IComponent<TMessage>>,
    ) -> ComponentChain<TMessage> {
        let rx = self.next_rx.take();
        component.set_input(rx);
        self.components.push(component);
        ComponentChain::new(self.components)
    }

    pub fn add(mut self) -> Self {
        let node = match self.mode {
            States::Start => {
                self.mode = States::Normal;
                Node::new(0)
            }
            States::Normal => {
                let id = self.nodes.len();
                let prev_id = id - 1;
                self.nodes[prev_id].next.push(id);
                let mut node = Node::new(id);
                node.prev.push(prev_id);
                node
            }
            States::Split => todo!(),
            States::Branch => todo!(),
            States::Join => todo!(),
        };
        self.nodes.push(node);
        self
    }

    pub fn split(mut self) -> Self {
        self.mode = States::Split;
        self
    }

    pub fn branch(mut self) -> Self {
        self
    }

    pub fn join(mut self) -> Self {
        self
    }
}

#[derive(Debug)]
struct Node {
    id: usize,
    prev: Vec<usize>,
    next: Vec<usize>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            prev: vec![],
            next: vec![],
        }
    }
}

enum States {
    Start,
    Normal,
    Split,
    Branch,
    Join,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Deserialize, Serialize)]
    enum TestMessage {}

    impl IMessage for TestMessage {}

    #[test]
    fn test1() {
        let chain = ComponentChainBuilder::<TestMessage>::new(100)
            .add()
            .add()
            .add();

        println!("{:?}", chain.nodes);
    }
}
