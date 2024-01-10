use tracing::info;

/// Настройка логгирования для платформы WASM32
#[cfg(target_arch = "wasm32")]
pub fn configure_logging(level: &str) {
    use tracing_subscriber::{
        fmt::{format::Pretty, time::ChronoLocal},
        prelude::*,
        EnvFilter,
    };
    use tracing_web::{performance_layer, MakeWebConsoleWriter};

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(ChronoLocal::rfc_3339())
        .with_writer(MakeWebConsoleWriter::new());
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .with(EnvFilter::new(level))
        .init();

    info!("service started");
}

// TODO - переделать текстовый параметр в tracing::Level
