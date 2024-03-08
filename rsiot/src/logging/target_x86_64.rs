use std::env;

use tokio::spawn;
use tracing::info;
use tracing_subscriber::{
    fmt::Layer, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

/// Настройка логгирования
///
/// Логгирование настраивается через входной параметр `rust_log`.
///
/// Логи выводятся в:
/// - stdout
/// - Grafana Loki
pub async fn configure_logging(rust_log: &str, loki_url: &url::Url) -> Result<(), super::Error> {
    let service = env::args().collect::<Vec<String>>()[0].clone();
    let service = service_cleanup(&service);

    // архивируем в Loki
    let (layer_loki, task_loki) = tracing_loki::builder()
        .label("service", service.clone())?
        .build_url(loki_url.clone())?;

    // архивируем в консоль
    let layer_stdout = Layer::new().pretty();

    // фильтруем на основе значения переменной RUST_LOG
    let filter = EnvFilter::new(rust_log);

    registry()
        .with(layer_loki)
        .with(layer_stdout)
        .with(filter)
        .init();

    spawn(task_loki);

    info!("service {} started", service);
    Ok(())
}

/// Удалить путь из названия файла
fn service_cleanup(input: &str) -> String {
    input.split('/').last().unwrap().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!("service", service_cleanup("./service"));
        assert_eq!("service", service_cleanup("../dir/service"));
        assert_eq!("service", service_cleanup("service"));
        assert_eq!("", service_cleanup(""));
    }
}
