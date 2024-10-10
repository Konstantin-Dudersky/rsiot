//! SystemD
//!
//! Пример создания файла для автозапуска сервисов с помощью SystemD
//!
//! Файл _PROJECT_.service:
//!
//! ```service
#![doc = include_str!("project.service")]
//! ```
//!
//! Установить сервис на целевой машине:
//!
//! ```bash
//! sudo mv _PROJECT_.service /etc/systemd/system
//! sudo systemctl daemon-reload
//! sudo systemctl enable _PROJECT_
//! sudo systemctl start _PROJECT_
//! ```
