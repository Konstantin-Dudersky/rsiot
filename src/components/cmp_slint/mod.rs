//! Компонент для взаимодействия с библиотекой пользовательского интерфейса Slint
//!
//! **cargo apk**
//!
//! ```bash
//! cargo install cargo-apk
//! sudo apt install qtbase5-dev
//! ```
//!
//! **Java**
//!
//! ```bash
//! sudo apt install openjdk-21-jdk
//! ```
//!
//! Или более новую, что есть в репозиториях
//!
//! ```
//! $env.JAVA_HOME = /snap/android-studio/current/jbr/
//!
//! $env.ANDROID_HOME = "/home/konstantin/Android/Sdk"
//! $env.ANDROID_NDK_ROOT = $"($env.ANDROID_HOME)/ndk/26.2.11394342"
//!
//! $env.TOOLCHAIN = $"($env.ANDROID_NDK_ROOT)/toolchains/llvm/prebuilt/linux-x86_64"
//! $env.TARGET = aarch64-linux-android
//! $env.API = 33
//!
//! $env.AR = $"($env.TOOLCHAIN)/bin/llvm-ar"
//! $env.CC = $"($env.TOOLCHAIN)/bin/($env.TARGET)($env.API)-clang"
//! $env.AS = $env.CC
//! $env.CXX = $"($env.TOOLCHAIN)/bin/($env.TARGET)($env.API)-clang++"
//! $env.LD = $"($env.TOOLCHAIN)/bin/ld"
//! $env.RANLIB = $"($env.TOOLCHAIN)/bin/llvm-ranlib"
//! $env.STRIP = $"($env.TOOLCHAIN)/bin/llvm-strip"
//! ```
//!
//! Задать переменные в файле `.cargo/config.toml`:
//!
//! ```toml
//! [env]
//! ANDROID_HOME = "/home/konstantin/Android/Sdk"
//! ANDROID_NDK_ROOT = "/home/konstantin/Android/Sdk/ndk/26.2.11394342"
//! JAVA_HOME = "/snap/android-studio/current/jbr/"
//!
//! CC_aarch64-linux-android = "/home/konstantin/Android/Sdk/ndk/26.2.11394342/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"
//! CCX_aarch64-linux-android = "/home/konstantin/Android/Sdk/ndk/26.2.11394342/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang"
//! AR_aarch64-linux-android = "/home/konstantin/Android/Sdk/ndk/26.2.11394342/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar"
//! RANLIB_aarch64-linux-android = "/home/konstantin/Android/Sdk/ndk/26.2.11394342/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ranlib"
//! ```
//!
//! Проверить путь установки, версию NDK (26.2.11394342), версию платформы (34).

mod component;
mod config;
mod error;
mod fn_process;
mod tasks;

pub use component::Cmp;
pub use config::{Config, OutputSender};
pub use error::Error;

type Result<TMsg> = std::result::Result<TMsg, Error>;
type SlintWindow<TMainWindow> = slint::Weak<TMainWindow>;
