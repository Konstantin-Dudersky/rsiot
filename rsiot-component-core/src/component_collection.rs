use tokio::{
    spawn,
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use rsiot_messages_core::IMessage;
use tracing::error;

use crate::{error::ComponentError, Cache, IComponent};

/// Объединение компонентов в одну цепочку
///
/// # Пример
/// ```
#[doc = include_str!("../examples/example1.rs")]
/// ```
pub struct ComponentCollection<TMessage>
where
    TMessage: IMessage,
{
    /// Размер буфера сообщений в канале
    buffer_size: usize,
    /// Вектор всех компонентов
    components: Vec<Box<dyn IComponent<TMessage>>>,
}

impl<TMessage> ComponentCollection<TMessage>
where
    TMessage: IMessage + 'static,
{
    /// Создание коллекции компонентов
    pub fn new(buffer_size: usize, components: Vec<Box<dyn IComponent<TMessage>>>) -> Self {
        Self {
            buffer_size,
            components,
        }
    }

    /// Запустить на выполнение все компоненты. Поток ожидает выполения всех задач
    ///
    /// TODO - если добавится функциональность добавления JoinHandle в JoinSet, переделать
    /// https://github.com/tokio-rs/tokio/issues/5924
    pub async fn spawn(&mut self) -> Result<(), ComponentError> {
        let (input_tx, _input_rx) = broadcast::channel(self.buffer_size);
        let (output_tx, output_rx) = mpsc::channel(self.buffer_size);
        let cache = Cache::new();
        let mut task_set = JoinSet::new();

        let task_internal_handle = spawn(task_internal(output_rx, input_tx.clone(), cache.clone()));
        task_set.spawn(task_internal_handle);

        for component in self.components.iter_mut() {
            component.set_input(input_tx.subscribe());
            component.set_output(output_tx.clone());
            component.set_cache(cache.clone());
        }

        while let Some(mut cmp) = self.components.pop() {
            let task_component_handle = cmp.spawn()?;
            task_set.spawn(task_component_handle);
        }

        // Компоненты не должны заканчивать выполнение. Если хоть один остановился (неважно по какой
        // причине - по ошибке или нормально), это ошибка выполнения.
        let msg;
        if let Some(result) = task_set.join_next().await {
            match result {
                Ok(result) => match result {
                    Ok(result) => match result {
                        Ok(_) => {
                            msg = "Component has finished executing".to_string();
                        }
                        Err(err) => {
                            msg = format!("Component has finished executing with error: {:?}", err);
                        }
                    },
                    Err(err) => {
                        msg = format!("Component has finished executing with error: {:?}", err);
                    }
                },
                Err(err) => {
                    msg = format!("Component has finished executing with error: {:?}", err);
                }
            };
            error!(msg);
            return Err(ComponentError::Execution(msg));
        }
        Ok(())
    }
}

async fn task_internal<TMessage>(
    mut input: mpsc::Receiver<TMessage>,
    output: broadcast::Sender<TMessage>,
    cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage,
{
    while let Some(msg) = input.recv().await {
        let key = msg.key().clone();
        let value = msg.clone();
        {
            let mut lock = cache.write().await;
            let value_from_cache = lock.get(&key);
            if let Some(value_from_cache) = value_from_cache {
                // если значение эквивалентно сохраненному в кеше, переходим к ожиданию следующего
                // сообщения
                if value == *value_from_cache {
                    continue;
                }
            }
            lock.insert(key, value);
        }
        output.send(msg).map_err(|err| {
            let err = format!("internal component send to channel error, {:?}", err);
            ComponentError::Initialization(err)
        })?;
    }
    Ok(())
}
