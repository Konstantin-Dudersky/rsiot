use rsiot_messages_core::IMessage;
use tokio::task::JoinSet;

use crate::{component_chain_builder::ComponentChainBuilder, IComponent};

/// Объединение компонентов в одну цепочку
pub struct ComponentChain<TMessage>
where
    TMessage: IMessage,
{
    components: Vec<Box<dyn IComponent<TMessage>>>,
}

impl<TMessage> ComponentChain<TMessage>
where
    TMessage: IMessage,
{
    /// Создание цепочки. Создавать лучше через построитель и метод `init()`
    pub fn new(cmps: Vec<Box<dyn IComponent<TMessage>>>) -> Self {
        Self { components: cmps }
    }

    /// Инициализация построителя
    pub fn init(buffer: usize) -> ComponentChainBuilder<TMessage> {
        ComponentChainBuilder::new(buffer)
    }

    /// Запустить на выполнение все компоненты. Поток ожидает выполения
    /// всех задач
    ///
    /// TODO - обработка ошибок
    pub async fn spawn(&mut self) {
        let mut set = JoinSet::new();
        for cmp in self.components.iter_mut() {
            set.spawn(cmp.spawn());
        }
        while (set.join_next().await).is_some() {}
    }
}
