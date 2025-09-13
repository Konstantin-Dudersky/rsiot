//! Differential

mod output_value;
mod task;

pub use output_value::OutputValue;
pub(crate) use task::Task;

use super::{Error, IntMsgBound, Result};
