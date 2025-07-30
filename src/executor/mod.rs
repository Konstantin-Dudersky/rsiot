//! Исполнитель - структура для запуска компонентов
//!
//! # Пример с WASM и cmp_leptos
//!
//! ```rust
#![doc = include_str!("./test/executor_wasm_leptos.rs")]
//! ```

mod cache;
mod check_capacity;
mod cmp_in_out;
mod component;
mod component_executor;
mod error;
mod join_set_spawn;
mod sleep;
mod tokio_runtime_metrics;
mod types;

#[cfg(test)]
mod test;

pub use cache::Cache;
pub use check_capacity::CheckCapacity;
pub use cmp_in_out::CmpInOut;
pub use component::{Component, IComponentProcess};
pub use component_executor::{ComponentExecutor, ComponentExecutorConfig};
pub use error::ComponentError;
pub use join_set_spawn::join_set_spawn;
pub use sleep::sleep;
pub use tokio_runtime_metrics::TokioRuntimeMetrics;
pub use types::CmpResult;
