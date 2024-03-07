//! Компоненты для построения системы сбора и обработки данных
//!
//! [Документация](https://docs.rs/rsiot/latest/rsiot)
//!
//! Поддерживаемые архитектуры ([подробнее](https://doc.rust-lang.org/rustc/platform-support.html)):
//!
//! - x86_64-unknown-linux-gnu - 64-bit Linux - использование в бекенд (target_arch = "x86_64")
//! - aarch64-unknown-linux-gnu - ARM64 Linux - использование в бекенд (target_arch = "aarch64")
//! - wasm32-unknown-unknown - WebAssembly - для создания веб-интерфейсов (target_arch = "wasm32")
//! - riscv32imc-esp-espidf - RISC-V ESP-IDF - микроконтроллеры ESP32 на базе процессора RISC-V
//!   (target_arch = "riscv32")
//!
//! ## Зачем это надо
//!
//! ## Обзор
//!
#![doc = include_str!("../doc/Новая концепция-2024-01-03-10-46.svg")]
//! ![](./rsiot/doc/Новая%20концепция-2024-01-03-10-46.svg)
//!
//! ## Компоненты
//!
//! #### Взаимодействие с устройствами нижнего уровня
//!
//! [**modbus-client**](https://docs.rs/rsiot-modbus-client/latest/rsiot_modbus_client/cmp_modbus_client)
//!
//! Взаимодейтсвие с устройствами, поддерживающими протокол Modbus TCP сервер / Modbus RTU slave.
//!
//! [**http-client**](https://docs.rs/rsiot-http-client/latest/rsiot_http_client/cmp_http_client)
//!
//! Взаимодействие с устройствами, имеющими HTTP API.
//!
//! [**websocket-client**](https://docs.rs/rsiot-websocket-client/latest/rsiot_websocket_client/cmp_websocket_client)
//!
//! Взаимодействие с устройствами, поддерживющими функциональность Websocket сервера.
//!
//! TODO **opcua-client**
//!
//! Взаимодействие с контроллерами, имеющими функциональность OPC UA сервера.
//!
//! TODO **s7-client**
//!
//! Взаимодействие с контроллерами Siemens по протоколу S7.
//!
//! [**esp**](https://docs.rs/rsiot-esp/latest/rsiot_esp)
//!
//! Компоненты для взаимодействия с HAL (hardware access level) микроконтроллера ESP32.
//!
//! #### Взаимодействие с системами верхнего уровня
//!
//! [**http-server**](https://docs.rs/rsiot-http-server/latest/rsiot_http_server/cmp_http_server)
//!
//! Поддержка HTTP API, через который внешние клиенты могут получать и вводить данные.
//!
//! [**websocket-server**](https://docs.rs/rsiot-websocket-server/latest/rsiot_websocket_server/cmp_websocket_server)
//!
//! Поддержка Websocket сервера, к которому могут подключаться внешние клиенты.
//!
//! TODO **telegram**
//!
//! #### Брокеры сообщений
//!
//! [**redis-client**](https://docs.rs/rsiot-redis-client/latest/rsiot_redis_client/cmp_redis_client)
//!
//! Подписка и публикация сообщения в Redis.
//!
//! TODO **mqtt**
//!
//! #### Сохранение данных в БД
//!
//! [**timescaledb-storing**](https://docs.rs/rsiot-timescaledb-storing/latest/rsiot_timescaledb_storing/cmp_timescaledb_storing)
//!
//! Сохрание сообщений в виде временных рядов в TimescaleDB.
//!
//! #### Интерфейсы пользователя
//!
//! [**leptos**](https://docs.rs/rsiot-leptos/latest)
//!
//! Веб-интерфейс. Используется фреймворк leptos.
//!
//! #### Вспомогательные крейты
//!
//! [**plc**](https://docs.rs/rsiot-plc/latest)
//!
//! Выполнение произвольной логики в "стиле PLC".
//!
//! [**env-vars**](https://docs.rs/rsiot-env-vars/latest)
//!
//! Чтение конфигурации из файла `.env`.
//!
//! [**logging**](https://docs.rs/rsiot-logging/latest)
//!
//! Настройка логгирования
//!
//! ## Описание
//!
//! **Компоненты** представляют собой асинхронные функции. У всех функций три аргумента:
//!
//! ```rust
//! # use tokio;
//! # use rsiot_messages_core::IMessage;
//! async fn component<TMessage, TConfig>(
//!     input: Option<tokio::sync::mpsc::Receiver<TMessage>>,
//!     output: Option<tokio::sync::mpsc::Sender<TMessage>>,
//!     config: TConfig,
//! ) -> ()
//! where
//!     TMessage: IMessage
//! {}
//! ```
//!
//! Сообщения между компонентами передаются через каналы "many producers to a single consumer"
//! библиотеки `tokio`.
//!
//! Входной или выходной потоки могут быть не заданы, поэтому каналы обернуты в Option.
//!
//! Структура конфигурации типа `TConfig` у каждого компонента своя.
//!
//! Компоненты ничего не возвращают (точнее, возвращают тип `()`). Если в компоненте возникает
//! ошибка, логику перезапуска необходимо реализовать внутри данной функции. TODO - пересмотреть,
//! возможно стоит возвращать Result при критических ошибках.
//!
//! **Сообщения** представляют собой тип enum, например:
//!
//! ```rust
//! use rsiot_messages_core::eav::EavModel;
//! use rsiot_messages_core::IMessage;
//! use serde::{Deserialize, Serialize};
//!
//! [derive(Clone, Debug, Deserialize, Serialize)]
//! enum Message {
//!     /// Текущее значение температуры
//!     Temperature(f64),
//!     /// Задание уставки
//!     ChangeSetpoint(f64),
//! }
//!
//! impl IMessage for Message {
//!     fn into_eav(self) -> Vec<EavModel> {
//!         vec![]
//!     }}
//! ```
//!
//! Трейт `IMessage` реализует основные методы - см. документацию по крейту
//! [rsiot-messages-core](https://docs.rs/rsiot-messages-core/latest)
//!
//! Для упрощения компоненты можно создавать и объединять в **цепочку компонентов**.
//!
//! TODO - компонент для симуляции
//!
//! - может генерировать сообщения как на основе входных сообщений
//! - может генерировать сообщения периодически
//!
//!  ## Флаги `feature`:
#![doc = document_features::document_features!()]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
//! TODO Написать про роутинг сообщений

pub mod message {
    pub use rsiot_messages_core::*;
}

#[cfg(feature = "components")]
pub mod component_core {
    pub use rsiot_component_core::*;
}

pub mod components;

#[cfg(feature = "env-vars")]
pub mod env_vars {
    pub use rsiot_env_vars::*;
}

pub mod logging {
    pub use rsiot_logging::*;
}

/// Реэкспорт необходимых модулей
pub mod reexport {
    pub use chrono;
    #[cfg(feature = "components")]
    pub use tokio;
    pub use url;
}
