//! Компоненты для построения системы сбора, обработки и визуализации данных
//!
//! Документация по платформам:
//!
//! - [x86_64-unknown-linux-gnu](../../x86_64-unknown-linux-gnu/rsiot/index.html)
//! - [aarch64-linux-android](../../aarch64-linux-android/rsiot/index.html)
//! - [aarch64-unknown-linux-gnu](../../aarch64-unknown-linux-gnu/rsiot/index.html)
//! - [riscv32imc-esp-espidf](../../riscv32imc-esp-espidf/rsiot/index.html)
//! - [wasm32-unknown-unknown](../../wasm32-unknown-unknown/rsiot/index.html)
//!
//! ## Обзор
//!
//! Набор компонентов для создания системы управления и диспетчиризации.
//!
//! Клиентское подключение:
//!
//! - cmp_http_client_wasm
//! - cmp_http_client
//! - cmp_modbus_client
//! - cmp_websocket_client_wasm
//! - cmp_websocket_client
//!
//! Сервера:
//!
//! - cmp_http_server_esp
//! - cmp_http_server
//! - cmp_websocket_server
//!
//! Брокеры сообщений:
//!
//! - cmp_esp_mqtt_client
//! - cmp_mqtt_client
//! - cmp_redis_client
//!
//! Интерфейс пользователя:
//!
//! - cmp_leptos
//! - cmp_slint
//!
//! Авторизация:
//!
//! - cmp_auth
//!
//! Сохранение данных:
//!
//! - cmp_esp_nvs
//! - cmp_influxdb
//! - cmp_surrealdb
//! - cmp_timescaledb
//! - cmp_webstorage
//!
//! Взаимодействие с аппаратной частью:
//!
//! - cmp_esp_adc
//! - cmp_esp_gpio
//! - cmp_esp_i2c_master
//! - cmp_esp_i2c_slave
//! - cmp_esp_led
//! - cmp_esp_mqtt_client
//! - cmp_esp_spi_master
//! - cmp_esp_wifi
//! - cmp_raspberrypi_gpio
//!
//! Логика исполнения
//!
//! - cmp_plc
//!
//! Систеная информация
//!
//! - cmp_system_info
//!
//! Служебные компоненты:
//!
//! - cmp_add_input_stream
//! - cmp_add_output_stream
//! - cmp_derive
//! - cmp_external_fn_process
//! - cmp_inject_periodic
//! - cmp_logger
//!
#![doc = include_str!("../doc/Новая концепция-2024-01-03-10-46.svg")]
//! ![](./rsiot/doc/Новая%20концепция-2024-01-03-10-46.svg)
//!
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
//!
//! - может генерировать сообщения как на основе входных сообщений
//! - может генерировать сообщения периодически
//!
//!  ## Флаги `feature`:
#![doc = document_features::document_features!()]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(rustdoc::bare_urls)]
#![warn(missing_docs)]
// #![feature(iterator_try_collect)]

pub mod components_config;
pub mod message;

#[cfg(feature = "executor")]
pub mod components;

#[cfg(any(feature = "cmp_esp", feature = "cmp_raspberrypi"))]
pub mod drivers_i2c;

#[cfg(feature = "env_vars")]
pub mod env_vars;

#[cfg(feature = "executor")]
pub mod executor;

// #[cfg(feature = "logging")]
pub mod logging;

#[cfg(feature = "rustdoc")]
pub mod doc;

pub mod serde_utils;

mod utils;
