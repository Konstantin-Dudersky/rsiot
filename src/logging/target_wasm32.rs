use tracing::info;
use tracing_subscriber::{
    fmt::{format::Pretty, time::ChronoLocal},
    prelude::*,
    EnvFilter,
};
use tracing_web::{performance_layer, MakeWebConsoleWriter};

/// Настройка логгирования для платформы WASM32
///
/// Логи выводятся в консоль
pub fn configure_logging(rust_log: &str) -> super::Result<()> {
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

    Ok(())
}
