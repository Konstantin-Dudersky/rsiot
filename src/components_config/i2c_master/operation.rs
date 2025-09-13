use std::time::Duration;

// ANCHOR: Operation
/// Виды операций
#[derive(Clone, Debug)]
pub enum Operation {
    /// Задержка между операциями
    Delay {
        /// Значение задержки
        delay: Duration,
    },

    /// Запрос записи и  чтения. Вложенные данные - количество байт для чтения
    WriteRead {
        /// Данные для записи
        write_data: Vec<u8>,
        /// Количество байт для чтения
        read_size: u8,
    },

    /// Запрос записи
    Write {
        /// Данные для записи
        write_data: Vec<u8>,
    },

    /// Запрос чтения
    Read {
        /// Количество байт для чтения
        read_size: u8,
    },
}
// ANCHOR: Operation
