use std::time::Duration;

// ANCHOR: Operation
/// Виды операций
#[derive(Clone, Debug)]
pub enum Operation {
    /// Задержка между операциями
    Delay(Duration),

    /// Запрос чтения
    Read {
        /// Количество байт для чтения
        read_size: u8,
    },

    /// Запрос записи и чтения. Вложенные данные - количество байт для чтения
    WriteRead(Vec<u8>, u8),

    /// Запрос записи. Вложенные данные - массив байт для записи
    Write(Vec<u8>),
}
// ANCHOR: Operation
