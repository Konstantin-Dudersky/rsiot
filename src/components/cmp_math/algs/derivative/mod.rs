//! Differential

mod calculation;
mod gamma;
mod output_value;
mod task;

pub(crate) use task::Task;
pub use {gamma::Gamma, output_value::OutputValue};

use super::{AlgInput, AlgOutput, Error, IntMsgBound, Result};
