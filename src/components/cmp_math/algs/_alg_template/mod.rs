//! Шаблон алгоритма

mod calculation;
mod output_value;
mod task;

pub use output_value::OutputValue;
pub(crate) use task::Task;

use super::{AlgFnOutputMsgbus, AlgInput, AlgOutput, Error};
