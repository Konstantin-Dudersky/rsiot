use rsiot_messages_core::IMessage;
use tokio::{
    spawn,
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{cmpbase_mpsc_to_broadcast, IComponent};

/// Объединение компонентов в одну цепочку
///
/// # Пример
/// ```
#[doc = include_str!("../examples/example1.rs")]
/// ```

pub struct ComponentChain<TMessage>
where
    TMessage: IMessage,
{
    /// Размер буфера сообщений в канале
    buffer_size: usize,
    /// Вектор всех компонентов
    components: Vec<Box<dyn IComponent<TMessage>>>,
}

impl<TMessage> ComponentChain<TMessage>
where
    TMessage: IMessage + 'static,
{
    /// Создание цепочки
    pub fn new(buffer_size: usize, components: Vec<Box<dyn IComponent<TMessage>>>) -> Self {
        Self {
            buffer_size,
            components,
        }
    }

    /// Запустить на выполнение все компоненты. Поток ожидает выполения всех задач
    pub async fn spawn(&mut self) {
        let (input_tx, _input_rx) = broadcast::channel(self.buffer_size);
        let (output_tx, output_rx) = mpsc::channel(self.buffer_size);

        for component in self.components.iter_mut() {
            component.set_input(input_tx.subscribe());
            component.set_output(output_tx.clone());
        }

        let mut set = JoinSet::new();
        while let Some(mut cmp) = self.components.pop() {
            set.spawn(cmp.spawn());
        }
        let _task = cmpbase_mpsc_to_broadcast::new(output_rx, input_tx);
        spawn(_task);

        while (set.join_next().await).is_some() {}
    }
}
