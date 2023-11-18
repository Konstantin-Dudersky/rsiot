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
        }
    }

    /// Начинаем цепочку
    pub fn start_with(
        mut self,
        mut component: Box<dyn IComponent<TMessage>>,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<TMessage>(self.buffer);
        component.set_stream_output(Some(tx));
        self.next_rx = Some(rx);
        self.components.push(component);
        self
    }

    /// Продолжаем цепочку
    pub fn then_with(
        mut self,
        mut component: Box<dyn IComponent<TMessage>>,
    ) -> Self {
        let (tx, rx) = mpsc::channel::<TMessage>(self.buffer);
        let next_rx = self.next_rx.take();
        component.set_stream_input(next_rx);
        component.set_stream_output(Some(tx));
        self.next_rx = Some(rx);
        self
    }

    /// Заканчиваем цепочку, возращаем готовую коллекцию компонентов
    pub fn end_with(
        mut self,
        mut component: Box<dyn IComponent<TMessage>>,
    ) -> ComponentChain<TMessage> {
        let rx = self.next_rx.take();
        component.set_stream_input(rx);
        self.components.push(component);
        ComponentChain::new(self.components)
    }
}
