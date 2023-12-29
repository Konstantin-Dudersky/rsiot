#![allow(unused_imports)]

mod error;

use std::env;

use tracing::info;
use url::Url;

pub use error::Error;

/// Настройка логгирования
///
/// Уровни логгирования настраиваются в переменной `RUST_LOG`
///
/// Потребители логов:
/// - stdout (в режиме Debug)
/// - Grafana Loki
#[cfg(target_arch = "x86_64")]
pub async fn configure_logging(loki_url: &Url) -> Result<(), Error> {
    use tokio::spawn;
    use tracing_subscriber::{prelude::*, EnvFilter};

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

#[cfg(target_arch = "wasm32")]
pub fn configure_logging() {}
