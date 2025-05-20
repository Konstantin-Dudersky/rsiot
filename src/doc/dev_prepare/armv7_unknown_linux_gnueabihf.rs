//! # armv7-unknown-linux-gnueabihf
//!
//! Ставим библиотеки:
//!
//! ```bash
//! sudo apt sudo apt install gcc-arm-linux-gnueabihf
//!
//! # Возможно добавить g++-arm-linux-gnueabihf libc6-dev-armhf-cross
//! ```
//!
//! Добавляем таргет:
//!
//! ```bash
//! rustup target add armv7-unknown-linux-gnueabihf
//! ```
//!
//! В проекте создаем файл `.cargo/config.toml`:
//!
//! ```toml
//! [target.armv7-unknown-linux-gnueabihf]
//! linker = "arm-linux-gnueabihf-gcc"
//! rustflags = ["-C", "target-feature=+crt-static"]
//! ```
