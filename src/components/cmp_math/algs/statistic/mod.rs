//! Шаблон алгоритма

mod calculation;
mod indicators;
mod output_value;
mod task;

#[allow(unused)]
pub(crate) use task::Task;
pub use {indicators::Indicators, output_value::OutputValue};

use super::{AlgFnOutputMsgbus, AlgInput, AlgOutput, Error};
