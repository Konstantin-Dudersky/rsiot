//! # Grafana
//!
//! ## docker
//! ```yaml
#![doc = include_str!("docker-compose.yml")]
//! ```
//!
//! ## Файлы конфигурации
//!
//! ### `svc_grafana/datasources/`
//!
//! В папке хранятся файлы для настройки источников данных.
//!
//! `svc_grafana/datasources/influxdb.yaml:`
//! ```yaml
#![doc = include_str!("datasources/influxdb.yaml")]
//! ```
//!
//! `svc_grafana/datasources/loki.yaml:`
//! ```yaml
#![doc = include_str!("datasources/loki.yaml")]
//! ```
//!
//! `svc_grafana/datasources/timescaledb.yaml:`
//! ```yaml
#![doc = include_str!("datasources/timescaledb.yaml")]
//! ```
//!
//! ### `svc_grafana/datasources/dashboards/`
//!
//! В папке хранятся все дашбоарды. Структура папок переносится в структуру дашбоардов. В корне папки нужно разместить файл config.yaml:
//!
//! ```yaml
#![doc = include_str!("dashboards/config.yaml")]
//! ```
