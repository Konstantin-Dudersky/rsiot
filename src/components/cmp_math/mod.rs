//! Комонент для математической обработки данных

mod algs;
mod component;
mod config;
mod error;
mod fn_process;
mod task_input;
mod task_output;

pub use algs::{Algs, EmaKind, Gamma, downsampling, statistic};
pub use component::{COMPONENT_NAME, Cmp};
pub use config::{Config, ConfigBranch};
pub use error::Error;
