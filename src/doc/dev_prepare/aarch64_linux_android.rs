//! # aarch64-linux-android
//!
//! **Таргет**
//!
//! ```bash
//! rustup target add aarch64-linux-android
//! ```
//!
//! **Android Studio**
//!
//! ```bash
//! sudo snap install android-studio --classic
//! ```
//!
//! Запустить android studio, поставить пакеты по-умолчанию. Открыть SDK Manager и установить:
//!
//! - Android SDK Platform
//! - Android SDK Build-Tools
//! - NDK (Side by side)
//! - Android SDK CLI
//! - Android SDK Platform-Tools
//!
//! **ADB**
//!
//! ```bash
//! sudo apt install adb
//! ```
//!
//! ## Отладка
//!
//! Получить отфильтрованные логи из телефона:
//!
//! ```bash
//! adb logcat RustStdoutStderr:I *:S
//! ```
//!
//! Выводить только сообщения от Rust с уровнем Info и выше.
//!
//! Уровни фильтрации:
//!
//! - V: Verbose (lowest priority)
//! - D: Debug
//! - I: Info
//! - W: Warning
//! - E: Error
//! - F: Fatal
//! - S: Silent (highest priority, where nothing is ever printed)
