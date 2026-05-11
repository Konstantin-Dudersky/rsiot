//! Компонент для обмена данными со сторонними системами по протоколу HTTP.
//!
//! Версия для выполнения на платформах x86-64 и ARM. Реализован на основе крейта reqwest.
//!
//! Можно выполнять GET, POST, PUT и DELETE методы HTTP.
//!
//! # Структура
//!
#![doc = include_str!("doc/diagram.svg")]
//!
//! Компонент состоит из одной задачи InjectPeriodic. С периодом [Config::period] на основе функции
//! [Config::fn_periodic] формируются исходящие сообщения и передаются в шину MsgBus.
//!
//! | Название | Описание |
//! |----------|----------|
//! | Input | Принимает сообщения из шины сообщений MsgBus. На основе данных формируются HTTP-запросы. Запросы перенаправляются в задачу HttpClient. Конфигурирование осуществляется с помощью структуры RequestInputConfig |
//! | Periodic | Задача периодически формирует HTTP-запросы и отправляет их в задачу HttpClient. Можно запусить несколько экземпляров. Конфигурирование осуществляется с помощью структуры RequestPeriodicConfig |
//! | HttpClient | Задача отправляет сформированные HTTP-запросы серверу. Реализация данной задачи отличается в разных компонентах (cmp_http_client, cmp_http_client_esp, cmp_http_client_wasm). Ответы от сервера отправляются в задачу Response |
//! | Response | Задача преобразует ответы от сервера в исходящие сообщения и отправляет в шину MsgBus. |
//!
//! # Конфигурация
//!
//! Конфигурация задаётся структурой [Config].
//!
//! # Примеры
//!
//! ## Пример 1
//!
//! Содержимое файла `config_http_client/mod.rs`:
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_http_server_and_client/cmp_http_client.rs")]
//! ```

mod component;
mod config;
mod fn_process;
mod tasks;

pub use crate::components::shared_tasks::cmp_http_client::Error;
pub use component::Cmp;
pub use config::*;

type Result<T> = std::result::Result<T, Error>;
