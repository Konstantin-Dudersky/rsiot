//! # Sentryshot
//!
//! Сохранение потока с видеокамеры. [Ссылка на репозиторий](https://github.com/SentryShot/sentryshot).
//!
//! ## docker
//!
//! ```yml
#![doc = include_str!("docker-compose.yml")]
//! ```
//!
//! ## Файлы конфигурации
//!
//! ### `./sentryshot/configs/sentryshot.toml`
//!
//! Проверить max_disk_usage.
//!
//! ```toml
#![doc = include_str!("configs/sentryshot.toml")]
//! ```
//!
//! ### `./sentryshot/configs/monitors/`
//!
//! В папке хранятся файлы конфигурации для каждой камеры. Пример файла 'tapo.json' для камеры RTSP:
//!
//! ```json
#![doc = include_str!("configs/monitors/tapo.json")]
//! ```
