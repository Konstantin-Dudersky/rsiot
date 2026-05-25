//! Компонент для взаимодействия с библиотекой пользовательского интерфейса Slint
//!
//! # Создание проекта
//!
//! ## Файл `Cargo.toml`
//!
//! ```toml
#![doc = include_str!("../../../examples_large/cmp_slint_minimal/Cargo.toml")]
//! ```
//!
//! В настройках крейта slint указываем требуемый бекенд и рендер. Для встраиваемых систем без
//! графического окружения лучше всего подойдёт бекенд `backend-linuxkms-noseat` и рендер
//! `renderer-skia`.
//!
//! ## Dockerfile
//!
//! Создаём в проекте файл Dockerfile.armv7-unknown-linux-gnueabihf (или аналогичный для другой
//! архитектуры). Содержимое файла нужно взять из репозитория slint -
//! https://github.com/slint-ui/slint/tree/master/docker.
//!
//! ## build.rs
//!
//! В файле build.rs прописываем функцию предкомпиляции
//!
//! ```rust
#![doc = include_str!("../../../examples_large/cmp_slint_minimal/build.rs")]
//! ```
//!
//! ## src/config_slint/app/main_window.slint
//!
//! Создаём файл main_window.slint с описанием основного окна.
//!
//! ```rust
#![doc = include_str!("../../../examples_large/cmp_slint_minimal/src/config_slint/app/main_window.slint")]
//! ```
//!
//! ## src/config_slint/app/global/input.slint
//!
//! Создаём файл input.slint. В этом файле прописывается структура Input, данные которой будут
//! обновляться на основе входящих сообщений из шины MsgBus.
//!
//! ```rust
#![doc = include_str!("../../../examples_large/cmp_slint_minimal/src/config_slint/app/global/input.slint")]
//! ```
//!
//! ## src/config_slint/app/global/output.slint
//!
//! Создаём файл output.slint. В этом файле прописывается структура Output с функциями
//! обратного вызова. Эти функции вызываются при событиях интерфейса - нажатие на кнопку, ввод
//! данных и т.д. На основе этих событий формируются исходящие сообщения в шину MsgBus.
//!
//! ```rust
#![doc = include_str!("../../../examples_large/cmp_slint_minimal/src/config_slint/app/global/output.slint")]
//! ```
//!
//! ## src/config_slint/app/global/main.slint
//!
//! Создаём файл main.slint. В этом файле делаем реэкспорт структур для использования в
//! компонете cmp_slint. Этот файл необходим, поскольку Rust может использовать данные только из
//! одного файла.
//!
//! ```rust
#![doc = include_str!("../../../examples_large/cmp_slint_minimal/src/config_slint/app/global/main.slint")]
//! ```
//!
//! ## Компоненты Material
//!
//! Клонировать репозиторий
//!
//! ```sh
//! git clone https://github.com/slint-ui/slint.git
//! ```
//!
//! и положить содержимое папки
//! ui-libraries/material/src в папку проекта src/config_slint/app/components/material
//!
//! ## .zed/settings.json
//!
//! Добавить путь к библиотеке material для работы LSP редактора Zed.
//!
//! ```json
#![doc = include_str!("../../../examples_large/cmp_slint_minimal/.zed/settings.json")]
//! ```
//!
//! ## src/config_slint/mod.rs
//!
//! Создаём файл mod.rs. В этом файле задаём конфигурацию компонента cmp_slint. Импорт данных из
//! Slint (структуры Input, Output, главное окно MainWindow) импортируются с помощью функции
//! slint::include_modules!
//!
//! ```rust
#![doc = include_str!("../../../examples_large/cmp_slint_minimal/src/config_slint/mod.rs")]
//! ```
//!
//!
//! ## src/main.rs
//!
//! Создаём файл src/main.rs. Интерфейс Slint должен запускаться в основном потоке, поэтому все
//! компоненты uScada запускаются в другом потоке с асинхронной средой выполнения.
//!
//! ```rust
#![doc = include_str!("../../../examples_large/cmp_slint_minimal/src/main.rs")]
//! ```
//!
//! # Остальное
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
