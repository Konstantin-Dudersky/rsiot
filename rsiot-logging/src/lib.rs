mod error;

use std::env;

use tokio::spawn;
use tracing::info;
use tracing_loki::url::Url;
use tracing_subscriber::{prelude::*, EnvFilter};

pub use error::Error;

/// Настройка логгирования
///
/// Уровни логгирования настраиваются в переменной `RUST_LOG`
///
/// Потребители логов:
/// - stdout (в режиме Debug)
/// - Grafana Loki
pub async fn configure_logging(loki_url: &Url) -> Result<(), Error> {
    let service = env::args().collect::<Vec<String>>()[0].clone();

    let (layer_loki, task) = tracing_loki::builder()
        .label("service", service.clone())?
        .build_url(loki_url.clone())?;

    // архивируем в консоль только в дебаг режиме
    let layer_stdout = match cfg!(debug_assertions) {
        true => Some(tracing_subscriber::fmt::Layer::new().pretty()),
        false => None,
    };

    tracing_subscriber::registry()
        .with(layer_loki)
        .with(layer_stdout)
        .with(EnvFilter::from_default_env())
        .init();

    spawn(task);

    info!("service {} started", service);
    Ok(())
}
