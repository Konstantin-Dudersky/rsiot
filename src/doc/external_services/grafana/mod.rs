//! # Grafana
//!
//! ## docker
//! ```yaml
#![doc = include_str!("docker-compose.yml")]
//! ```
//!
//! ## Файлы конфигурации
//!
//! ### `datasources/`
//!
//! В папке хранятся файлы для настройки источников данных.
//!
//! `influxdb.yaml:`
//! ```yaml
#![doc = include_str!("datasources/influxdb.yaml")]
//! ```
//!
//! `loki.yaml:`
//! ```yaml
#![doc = include_str!("datasources/loki.yaml")]
//! ```
//!
//! `timescaledb.yaml:`
//! ```yaml
#![doc = include_str!("datasources/timescaledb.yaml")]
//! ```
//!
//! ### `datasources/`
//!
//! В папке хранятся все дашбоарды. Структура папок переносится в структуру дашбоардов. В корне папки нужно разместить файл config.yaml:
//!
//! ```yaml
#![doc = include_str!("dashboards/config.yaml")]
//! ```
