//! Исполнитель - структура для запуска компонентов
//!
//! # Пример с WASM и cmp_leptos
//!
//! ```rust
#![doc = include_str!("../../examples/executor_wasm_leptos/main.rs")]
//! ```

mod cache;
mod check_capacity;
mod cmp_in_out;
mod component;
mod component_executor;
mod error;
mod instant;
mod join_set_spawn;
mod sleep;
mod tokio_runtime_metrics;
mod types;

pub use cache::Cache;
pub use check_capacity::CheckCapacity;
pub use cmp_in_out::CmpInOut;
pub use component::{Component, IComponentProcess};
pub use component_executor::{ComponentExecutor, ComponentExecutorConfig};
pub use error::ComponentError;
pub use instant::Instant;
pub use join_set_spawn::join_set_spawn;
pub use sleep::sleep;
pub use tokio_runtime_metrics::TokioRuntimeMetrics;
pub use types::CmpResult;
