//! Исполнитель - структура для запуска компонентов
//!
//! # Пример с WASM и cmp_leptos
//!
//! ```rust
#![doc = include_str!("../../examples/executor_wasm_leptos/main.rs")]
//! ```

mod cache;
mod check_capacity;
mod component;
mod component_executor;
mod error;
mod instant;
mod join_set_spawn;
mod less_in_period;
mod msgbus_input;
mod msgbus_linker;
mod msgbus_output;
mod sleep;
mod task_internal;
#[cfg(feature = "log_tokio")]
mod task_runtime_metrics;
mod tokio_runtime_metrics;
mod types;

pub use cache::Cache;
pub use check_capacity::CheckCapacity;
pub use component::{Component, IComponentProcess};
pub use component_executor::{ComponentExecutor, ComponentExecutorConfig};
pub use error::ComponentError;
pub use instant::Instant;
pub use join_set_spawn::join_set_spawn;
pub(crate) use less_in_period::LessInPeriod;
pub use msgbus_linker::MsgBusLinker;
pub use sleep::sleep;
pub use tokio_runtime_metrics::TokioRuntimeMetrics;
pub use types::CmpResult;
pub use {msgbus_input::MsgBusInput, msgbus_output::MsgBusOutput};
