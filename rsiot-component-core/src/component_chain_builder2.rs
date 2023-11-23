use tokio::sync::mpsc;

use rsiot_messages_core::IMessage;

use crate::{ComponentChain, IComponent, StreamInput};

type Id = usize;

/// Построитель для цепочки компонентов
pub struct ComponentChainBuilder<TMessage> {
    /// Размер буфера сообщений в канале
    buffer: usize,
    /// Коллекция компонентов
    components: Vec<Box<dyn IComponent<TMessage>>>,
    /// Используется для сохраниния приемной стороны канала сообщений
    next_rx: StreamInput<TMessage>,
    /// -- experiment
    prev_node: PrevNodeType,
    transitions: Vec<(Id, Id)>,
    split_node: Option<Id>,
    open_branshes: Vec<Id>,
    components2: Vec<()>,
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
            prev_node: PrevNodeType::NoNode,
            transitions: vec![],
            split_node: None,
            open_branshes: vec![],
            components2: vec![],
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
        let id = self.components2.len();

        self.components2.push(());

        match self.prev_node {
            PrevNodeType::NoNode => {
                self.prev_node = PrevNodeType::PrevNode(id);
            }
            PrevNodeType::PrevNode(prev_id) => {
                self.transitions.push((prev_id, id));
                self.prev_node = PrevNodeType::PrevNode(id);
            }
            PrevNodeType::OpenBranches(ids) => {
                for _id in ids {
                    self.transitions.push((_id, id));
                }
                self.prev_node = PrevNodeType::PrevNode(id);
            }
        }
        self
    }

    pub fn split(mut self) -> Self {
        self.split_node = Some(self.components2.len() - 1);
        self
    }

    pub fn branch(mut self) -> Self {
        self.prev_node = PrevNodeType::PrevNode(self.split_node.unwrap());
        self.open_branshes.push(self.components2.len() - 1);
        self
    }

    pub fn join(mut self) -> Self {
        self.open_branshes.push(self.components2.len() - 1);
        self.prev_node = PrevNodeType::OpenBranches(self.open_branshes.clone());
        self
    }
}

enum PrevNodeType {
    NoNode,
    PrevNode(usize),
    OpenBranches(Vec<usize>),
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
            .split()
            .add()
            .add()
            .branch()
            .add()
            .add()
            .branch()
            .add()
            .add()
            .add()
            .join()
            .add()
            .add();

        println!("{:?}", chain.transitions);
    }
}
