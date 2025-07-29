#![allow(clippy::needless_doctest_main)]
//! Настройки логгирования для разных платформ.
//!
//! Для настройки логгирования нужно задать переменную `RUST_LOG`.
//!
//! ## Способы задания RUST_LOG
//!
//! ### Запуск в контейнере
//!
//! В файле `docker-compose.yaml` для сервиса указать:
//!
//! ```yaml
//! services:
//!   rust_service:
//!     environment:
//!       - RUST_LOG=info
//! ```
//!
//! Значение переменной можно задавать для каждого сервиса оданиково.
//!
//! ### Запуск в контейнере, сохранение в файле `.env`
//!
//! В файле `docker-compose.yaml` для сервиса указать:
//!
//! ```yaml
//! services:
//!   rust_service:
//!     env_file: .env
//! ```
//!
//! Значение переменной будет одинаково для всех сервисов
//!
//! ### Задание в compile-time
//!
//! Платформы WASM, ESP не могут считывать переменные окружения, поэтому значение необходимо
//! прописывать на этапе компиляции.
//!
//! Чтобы значение переменной считывалось из файла:
//!
//! - создать файл .env в корне проекта
//! - прописать в файле переменную в виде `RUST_LOG = info`
//! - если изменить только переменную, без изменения кода, то перекомпиляции не будет. Поэтому можно
//!   создать файл `build.rs` в корне проекта с содержимым:
//!
//! ```rust
//! pub fn main() {
//!     println!("cargo:rerun-if-changed=.env");
//! }
//! ```
//!
//! TODO - Примеры задания переменной `RUST_LOG`
//!

mod error;
use std::env;

pub use error::Error;

// #[cfg(target_arch = "wasm32")]
// mod target_wasm32;
// #[cfg(target_arch = "wasm32")]
// pub use target_wasm32::configure_logging;

// #[cfg(any(
//     aarch64_unknown_linux_gnu,
//     armv7_unknown_linux_gnueabihf,
//     x8664_unknown_linux_gnu
// ))]
// mod target_x86_64;
// #[cfg(any(
//     aarch64_unknown_linux_gnu,
//     armv7_unknown_linux_gnueabihf,
//     x8664_unknown_linux_gnu
// ))]
// pub use target_x86_64::configure_logging;

// #[cfg(riscv32imc_esp_espidf)]
// mod target_esp;
// #[cfg(riscv32imc_esp_espidf)]
// pub use target_esp::configure_logging;

use tracing::info;
use tracing_subscriber::{
    fmt::Layer, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

type Result<T> = std::result::Result<T, Error>;

fn filter_value(filter: &LogConfigFilter) -> Result<String> {
    let filter = match filter {
        LogConfigFilter::FromEnv => env::var("RUST_LOG")?,
        LogConfigFilter::String(v) => v.to_string(),
    };
    Ok(filter)
}

/// Настройка логгирования
pub struct LogConfig {
    /// Строка с настройкой фильтрации логов
    #[cfg(any(
        feature = "log_console",
        feature = "log_loki",
        feature = "log_webconsole"
    ))]
    pub filter: LogConfigFilter,

    /// Адрес сервера Loki
    ///
    /// Пример:
    /// ```rust
    /// String::from("http://service_loki:3100")
    /// ```
    #[cfg(feature = "log_loki")]
    pub loki_url: String,

    /// Адрес для подключения tokio-console
    ///
    /// Пример:
    /// ```rust
    /// SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 6669)
    /// ```
    #[cfg(feature = "log_tokio")]
    pub tokio_console_addr: std::net::SocketAddrV4,

    #[cfg(feature = "log_esp")]
    pub esp_filter_level: String,
}
impl LogConfig {
    /// Запуск логгирования
    pub fn run(self) -> Result<()> {
        // log_console ---------------------------------------------------------------------------------
        #[cfg(feature = "log_console")]
        let layer_console = {
            use tracing_subscriber::{fmt, Layer};

            let global_filter = EnvFilter::new(filter_value(&self.filter)?);
            let layer = fmt::Layer::new().pretty().with_filter(global_filter);
            Some(layer)
        };
        #[cfg(not(feature = "log_console"))]
        let layer_console: Option<Layer<_>> = None;

        // log_loki ------------------------------------------------------------------------------------
        #[cfg(feature = "log_loki")]
        let layer_loki = {
            use tracing_subscriber::Layer;

            let global_filter = EnvFilter::new(filter_value(&self.filter)?);

            let service = "TODO";
            let loki_url = url::Url::parse(&self.loki_url)?;
            let (layer_loki, task_loki) = tracing_loki::builder()
                .label("service", service)?
                .build_url(loki_url.clone())?;
            tokio::spawn(task_loki);
            Some(layer_loki.with_filter(global_filter))
        };
        #[cfg(not(feature = "log_loki"))]
        let layer_loki: Option<Layer<_>> = None;

        // log_tokio -----------------------------------------------------------------------------------
        #[cfg(feature = "log_tokio")]
        let layer_tokio = {
            use tracing_subscriber::Layer;

            let filter = EnvFilter::new("tokio=trace,runtime=trace");
            let layer = console_subscriber::ConsoleLayer::builder()
                .server_addr(self.tokio_console_addr)
                .spawn()
                .with_filter(filter);

            // let layer = console_subscriber::spawn().with_filter(filter);
            Some(layer)
        };
        #[cfg(not(feature = "log_tokio"))]
        let layer_tokio: Option<Layer<_>> = None;

        // log_webconsole ------------------------------------------------------------------------------
        #[cfg(feature = "log_webconsole")]
        let layer_webconsole = {
            console_error_panic_hook::set_once();
            use tracing_subscriber::{fmt::time::ChronoLocal, Layer};
            use tracing_web::MakeWebConsoleWriter;

            let global_filter = EnvFilter::new(filter_value(&self.filter)?);

            let layer = tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_timer(ChronoLocal::rfc_3339())
                .with_writer(MakeWebConsoleWriter::new())
                .with_filter(global_filter);
            Some(layer)
        };
        #[cfg(not(feature = "log_webconsole"))]
        let layer_webconsole: Option<Layer<_>> = None;

        // log_webconsole_perf -------------------------------------------------------------------------
        #[cfg(feature = "log_webconsole")]
        let layer_webconsole_perf = {
            use tracing_subscriber::fmt::format::Pretty;
            use tracing_web::performance_layer;

            let layer = performance_layer().with_details_from_fields(Pretty::default());
            Some(layer)
        };
        #[cfg(not(feature = "log_webconsole"))]
        let layer_webconsole_perf: Option<Layer<_>> = None;

        // registry ------------------------------------------------------------------------------------
        registry()
            .with(layer_console)
            .with(layer_loki)
            .with(layer_tokio)
            .with(layer_webconsole)
            .with(layer_webconsole_perf)
            .init();

        #[cfg(feature = "log_loki")]
        info!("Logging in Loki started: {}", self.loki_url);

        #[cfg(feature = "log_tokio")]
        info!(
            "Logging in tokio-console on address: {}",
            self.tokio_console_addr
        );

        Ok(())
    }
}

/// Откуда брать строку с фильтрацией логов
pub enum LogConfigFilter {
    /// Из переменной окружения `RUST_LOG`
    FromEnv,
    /// Задать значение в строке
    String(&'static str),
}
