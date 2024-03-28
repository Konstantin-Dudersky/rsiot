use esp_idf_svc::log::{set_target_level, EspLogger};
use log::LevelFilter as LogLevelFilter;
use tracing::level_filters::LevelFilter as TracingLevelFilter;

/// Конфигурация логгирования для микроконтроллера ESP32
pub fn configure_logging(level: TracingLevelFilter) -> super::Result<()> {
    EspLogger::initialize_default();

    let level = match level {
        TracingLevelFilter::TRACE => LogLevelFilter::Trace,
        TracingLevelFilter::DEBUG => LogLevelFilter::Debug,
        TracingLevelFilter::INFO => LogLevelFilter::Info,
        TracingLevelFilter::WARN => LogLevelFilter::Warn,
        TracingLevelFilter::ERROR => LogLevelFilter::Error,
        TracingLevelFilter::OFF => LogLevelFilter::Off,
    };
    set_target_level("", level)?;
    Ok(())
}
