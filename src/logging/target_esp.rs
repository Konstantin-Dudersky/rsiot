use esp_idf_svc::log::{set_target_level, EspLogger};
use log::LevelFilter as LogLevelFilter;
use tracing::level_filters::LevelFilter as TracingLevelFilter;

/// Конфигурация логгирования для микроконтроллера ESP32
///
/// Также необходимо настроить переменную CONFIG_LOG_DEFAULT_LEVEL_ в sdkconfig.defaults. Возможные
/// значения:
/// - No output (CONFIG_LOG_DEFAULT_LEVEL_NONE)
/// - Error (CONFIG_LOG_DEFAULT_LEVEL_ERROR)
/// - Warning (CONFIG_LOG_DEFAULT_LEVEL_WARN)
/// - Info (CONFIG_LOG_DEFAULT_LEVEL_INFO)
/// - Debug (CONFIG_LOG_DEFAULT_LEVEL_DEBUG)
/// - Verbose (CONFIG_LOG_DEFAULT_LEVEL_VERBOSE)
///
/// В консоль будут попадать логи, которые являются минимумом двух значений:
/// - заданного в файле `sdkconfig.defaults` при компиляции
/// - значения аргумента `level` данной функции в рантайме
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
    set_target_level("*", level)?;
    Ok(())
}
