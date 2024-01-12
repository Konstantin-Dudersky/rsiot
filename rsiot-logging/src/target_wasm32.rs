/// Настройка логгирования для платформы WASM32
///
/// Переменная RUST_LOG должна задаваться в compile-time. Для этого
///
/// - создать файл .env в корне проекта
/// - прописать в файле переменную в виде `RUST_LOG = info`
/// - если изменить только переменную, без изменения кода, то перекомпиляции не будет. Поэтому можно
///   создать файл `build.rs` в корне проекта с содержимым:
///
/// ```rust
/// pub fn main() {
///     println!("cargo:rerun-if-changed=.env");
/// }
/// ```
pub fn configure_logging() {
    use tracing::info;
    use tracing_subscriber::{
        fmt::{format::Pretty, time::ChronoLocal},
        prelude::*,
        EnvFilter,
    };
    use tracing_web::{performance_layer, MakeWebConsoleWriter};

    let rust_log = dotenvy_macro::dotenv!("RUST_LOG");

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(ChronoLocal::rfc_3339())
        .with_writer(MakeWebConsoleWriter::new());
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    let filter = EnvFilter::new(rust_log);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .with(filter)
        .init();

    info!("service started");
}
