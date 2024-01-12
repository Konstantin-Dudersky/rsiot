/// Настройка логгирования
///
/// Логгирование настраивается через входной переменную окружения `rust_log`.
///
/// Логи выводятся в:
/// - stdout
/// - Grafana Loki
///
/// # Способы задания RUST_LOG
///
/// ## Запуск в контейнере
///
/// В файле `docker-compose.yaml` для сервиса указать:
///
/// ```yaml
/// services:
///   rust_service:
///     environment:
///       - RUST_LOG=info
/// ```
///
/// Значение переменной можно задавать для каждого сервиса оданиково.
///
/// ## Запуск в контейнере, сохранение в файле `.env`
///
/// В файле `docker-compose.yaml` для сервиса указать:
///
/// ```yaml
/// services:
///   rust_service:
///     env_file: .env
/// ```
///
/// Значение переменной будет одинаково для всех сервисов
pub async fn configure_logging(loki_url: &url::Url) -> Result<(), crate::Error> {
    use std::env;

    use tokio::spawn;
    use tracing::info;
    use tracing_subscriber::{
        fmt::Layer, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
    };

    let service = env::args().collect::<Vec<String>>()[0].clone();
    let service = service_cleanup(&service);

    // архивируем в Loki
    let (layer_loki, task_loki) = tracing_loki::builder()
        .label("service", service.clone())?
        .build_url(loki_url.clone())?;

    // архивируем в консоль
    let layer_stdout = Layer::new().pretty();

    // фильтруем на основе значения переменной RUST_LOG
    let filter = EnvFilter::from_default_env();

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
    input.split("/").last().unwrap().to_owned()
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
