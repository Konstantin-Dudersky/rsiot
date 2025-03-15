#![allow(rustdoc::bare_urls)]
//! Компонент для интеграции веб-приложения на основе фреймворка Leptos
//!
//! # Подготовка среды разработки
//!
//! ## Таргеты
//!
//! - [wasm32-unknown-unknown](`crate::doc::dev_prepare::wasm32_unknown_unknown`)
//! - [aarch64-linux-android](`crate::doc::dev_prepare::aarch64_linux_android`) (Tauri / Android)
//! - [armv7-linux-androideabi](`crate::doc::dev_prepare::armv7_linux_androideabi`) (Tauri / Android)
//! - [i686-linux-android](`crate::doc::dev_prepare::i686_linux_android`) (Tauri / Android)
//! - [x86_64-linux-android](`crate::doc::dev_prepare::x8664_linux_android`) (Tauri / Android)
//!
//! ## `.cargo/config.toml`
//!
//! ```toml
//! [target.wasm32-unknown-unknown]
//! rustflags = [
//!     "--cfg",
//!     "erase_components",
//! ]
//! ```
//!
//! ## Tauri
//!
//! Если требуется разработка приложения Tauri (https://beta.tauri.app/guides/prerequisites/).
//!
//! ```bash
//! (sudo apt install libwebkit2gtk-4.1-dev
//!   build-essential
//!   curl
//!   wget
//!   file
//!   libssl-dev
//!   libayatana-appindicator3-dev
//!   librsvg2-dev)
//! ```
//!
//! ### Tauri CLI
//!
//! ```bash
//! cargo install tauri-cli --version "^2.0.0-beta"
//! ```
//!
//! ### Create tauri apps
//!
//! ```bash
//! cargo install create-tauri-app
//! ```
//!
//! ### Android Studio
//!
//! ```bash
//! sudo snap install android-studio --classic
//! ```
//! Запустить Android Studio, поставить пакеты по-умолчанию. Открыть SDK Manager и установить NDK (Side by side) и Android SDK CLI.
//!
//! ### Env variables
//!
//! `~/.config/nushell/config.nu`:
//!
//! ```toml
//! $env.JAVA_HOME = "/snap/android-studio/current/jbr"
//! $env.ANDROID_HOME = "/home/konstantin/Android/Sdk"
//! $env.NDK_HOME = $"($env.ANDROID_HOME)/ndk/26.2.11394342"
//! ```
//!
//! Проверить пути
//!
//! ## Rustiwind
//!
//! Для сортировки классов Tailwind можно поставить утилиту:
//!
//! ```bash
//! cargo install rustywind
//! ```
//!
//! Вызов:
//!
//! ```bash
//! rustywind --write src/
//! ```
//!
//! ## Leptosfmt
//!
//! Установка
//!
//! ```bash
//! cargo install leptosfmt
//! ```
//!
//! # Создание проекта
//!
//! ##  `.zed/settings.json`
//!
//! ```json
//! {
//!   "lsp": {
//!     "rust-analyzer": {
//!       "initialization_options": {
//!         "check": {
//!           "command": "clippy"
//!         },
//!         "cargo": {
//!           "target": "wasm32-unknown-unknown"
//!         }
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! ## Tauri
//!
//! ```bash
//! cargo create-tauri-app --rc
//!
//! # добавить поддержку Android
//! cargo tauri android init
//! ```
//!
//! ##  Tailwind
//!
//! Установить:
//!
//! ```bash
//! npm install -D tailwindcss
//! npm install -D @tailwindcss/forms
//! npx tailwindcss init
//! ```
//!
//! Создать файл `tailwind.config.js`:
//!
//! ```json
//! /** @type {import('tailwindcss').Config} */
//!
//! module.exports = {
//!   content: {
//!     files:  [
//!       "*.html",
//!     "./src/**/*.rs",
//!     "../../rsiot/src/components/cmp_leptos/components/**/*.rs"
//!     ],
//!   },
//!   plugins: [require('@tailwindcss/forms'),],
//! }
//! ```
//!
//! Создать файл `input.css` в корне проекта:
//!
//! ```css
//! @tailwind base;
//! @tailwind components;
//! @tailwind utilities;
//! ```
//!
//! Добавить в `index.html`:
//!
//! ```html
//! <html>
//!   <head>
//!     <!-- Подключаем стили Tailwind -->
//!     <link data-trunk rel="tailwind-css" href="input.css" />
//!     <meta name="viewport" content="width=device-width, initial-scale=1.0" />
//!   </head>
//! </html>
//! ```
//!
//! ## Material Theme
//!
//! Создаем тему в [Material Theme Builder](https://material-foundation.github.io/material-theme-builder/).
//!
//! Скачиваем набор файлов css, распаковываем в папку `material-theme`. В начале файла `input.css` прописываем:
//!
//! ```css
//! /* Material theme */
//! @import "./material-theme/dark.css";
//! @import "./material-theme/dark-hc.css";
//! @import "./material-theme/dark-mc.css";
//! @import "./material-theme/light.css";
//! @import "./material-theme/light-hc.css";
//! @import "./material-theme/light-mc.css";
//! ```
//!
//! Прописать секцию `theme` в `tailwind.config.json`:
//!
//! ```json
#![doc = include_str!("./doc/tailwind.config.json")]
//! ```
//! Для выбора темы применяем класс к элементу `html.body`:
//!
//! ```html
//! <body class="dark"></body>
//! ```
//!
//! Допустимые классы:
//!
//! - dark-high-contrast
//! - dark-medium-contrast
//! - dark
//! - light-high-contrast
//! - light-medium-contrast
//! - light
//!
//!
//! Добавить в файл `input.css`:
//!
//! ```css
//! :root {
//!   --md-ref-typeface-brand: "Roboto";
//!   --md-ref-typeface-plain: system-ui;
//! }
//! ```
//!
//! Material theme builder почему-то не экспортирует настройки шрифтов. Когда пофиксят - пересмотреть.
//!
//! ## Iconify
//!
//! ```bash
//! npm i -D @iconify/tailwind
//! npm i -D @iconify/json
//! ```
//!
//! Добавить в файл `tailwind.config.js`:
//!
//! ```json
//! const { addIconSelectors } = require("@iconify/tailwind");
//!
//! module.exports = {
//!   plugins: [addIconSelectors(["mdi", "material-symbols"])],
//! }
//! ```
//!
//! Добавить в параметры `addIconSelectors` семейства иконок.
//!
//! Далее в проекте иконки можно вставлять:
//!
//! ```html
//! <span class="iconify material-symbols--menu-rounded h-5 w-5"></span>
//! ```
#![warn(rustdoc::bare_urls)]

mod component;
pub mod components;
pub mod create_signal_from_msg;
mod error;
mod global_state;
pub mod utils;

pub use component::{Cmp, Config, StoreBound};
pub use error::Error;
pub use global_state::GlobalState;
pub use rsiot_macros::create_signal_from_msg;

type Result = std::result::Result<(), Error>;
