use super::IMessageChannel;

/// Пример канала сообщения
#[derive(Clone, Debug)]
pub enum ExampleMessageChannel {
    /// Выходной канал - по умолчанию все компоненты отправляют данные сюда
    Output,
}

impl IMessageChannel for ExampleMessageChannel {}
