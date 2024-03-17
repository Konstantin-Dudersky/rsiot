//! Создание нового проекта для ESP32
//!
//! ```bash
//! cargo new project_name
//! ```
//!
//! Скопировать файлы из репозитория:
//! - rsiot/.cargo/config.toml
//! - rsiot/build.rs
//! - rsiot/rust-toolchain.toml
//! - rsiot/rust-toolchain.toml
//!
//! Добавить в Config.toml:
//!
//! ```toml
//! [build-dependencies]
//! embuild = { version = "*", features = ["espidf"] }
//! ```
//!
//! Первой строкой в функции main():
//!
//! ```rust
//! esp_idf_svc::sys::link_patches
//! ```
