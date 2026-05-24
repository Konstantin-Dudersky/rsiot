use std::time::Duration;

/// Сообщения для диагностики работы шины
pub enum FieldbusDiagMsg {
    /// Запрос по шине выполнен успешно
    FieldbusRequestOk {
        /// Длительность запроса
        duration: Duration,
    },

    /// Запрос по шине выполнен с ошибкой
    FieldbusRequestErr {
        /// Длительность запроса
        duration: Duration,
    },

    /// Инициализация устройства завершена
    DeviceInitCompleted {
        /// Идентификатор устройства
        id: String,

        /// Длительность запроса
        duration: Duration,
    },

    /// Запрос на устройство выполнен успешно
    DeviceRequestOk {
        /// Идентификатор устройства
        id: String,

        /// Длительность запроса
        duration: Duration,
    },

    /// Запрос на устройство выполнен с ошибкой
    DeviceRequestErr {
        /// Идентификатор устройства
        id: String,

        /// Длительность запроса
        duration: Duration,

        /// Описание ошибки
        error: String,
    },
}
