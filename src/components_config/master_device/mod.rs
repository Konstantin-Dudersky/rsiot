//! Реализация опроса подчиненных устройств по любому протоколу - I2C, SPI, UART
//!
//! На рисунке приведена структура работы драйвера устройства. В драйвере реализуется логическая
//! часть опроса устройств. Физическая реализация взамодействия с интерфейсом реализована в
//! конкретных компонентах. Таким образом, драйвера можно использовать в разных реализациях.
//! Например, драйвера устройств I2С можно использовать как в компоненте cmp_linux_i2c_master, так и
//! cmp_esp_i2c_master.
//!
#![doc = include_str!("doc/diagram.svg")]
//!
//! Цифрами 1, 2, 3, 4 помечены каналы передачи данных, соответствующие каналам на структуре
//! компонента опроса устройств ([FieldbusExecution](crate::components::shared_tasks::fieldbus_execution)). В таблице приведено описание задач.
//!
//! | Название | Описание |
//! | ------- | ------- |
//! | InitRequest | Задача создания запросов, с помощью которых можно инициализировать устройство. |
//! | PeriodicRequest | Задача создания периодических запросов. Можно создавать несколько запросов с разными периодами вызова. |
//! | Input | Обновление буфера на основе входящих сообщений из шины MsgBus. Обновление происходит на основе функции из конфигурации драйвера.|
//! | BufferPeriodic | Задача необходима для периодического запуска задачи BufferToRequests. |
//! | BufferToRequestsRequest | Задача создания запросов на основе буфера. |
//! | Request | Задача объединяет запросы из задач InitRequest, PeriodicRequest и BufferToRequests и отправляет запросы в интерфейс. |
//! | Response | Задача получает ответы из интерфейса. На основе конфигурации драйвера создаются исходящие сообщения и передаются в шину MsgBus. Также при необходимости можно передать сигнал в задачу BufferToRequests для создания новых запросов в интерфейс. |

mod buffer_bound;
mod device;
mod device_trait;
mod error;
mod fieldbus_diag;
mod fieldbus_request_with_index;
mod fieldbus_response_with_index;
mod request_response_bound;

pub(crate) use {
    fieldbus_request_with_index::FieldbusRequestWithIndex,
    fieldbus_response_with_index::FieldbusResponseWithIndex,
    request_response_bound::RequestResponseBound,
};

pub use {
    buffer_bound::BufferBound, device::*, device_trait::DeviceTrait, error::Error,
    fieldbus_diag::FieldbusDiagMsg,
};

/// Тип Result
pub type Result<T> = std::result::Result<T, Error>;
