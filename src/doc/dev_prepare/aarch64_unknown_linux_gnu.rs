//! # aarch64-unknown-linux-gnu
//!
//! Ставим библиотеки:
//!
//! ```bash
//! sudo apt install gcc-aarch64-linux-gnu g++-aarch64-linux-gnu
//! ```
//!
//! Добавляем таргет:
//!
//! ```bash
//! rustup target add aarch64-unknown-linux-gnu
//! ```
//!
//! В файле `~/.cargo/config.toml`:
//!
//! ```toml
//! [target.aarch64-unknown-linux-gnu]
//! linker = "aarch64-linux-gnu-gcc"
//! ```
