//! EMA - Exponential Moving Average

mod ema_kind;
mod output_value;
mod task;

pub use ema_kind::EmaKind;
pub use output_value::OutputValue;
pub(crate) use task::Task;

use super::{AlgFnOutputMsgbus, AlgInput, AlgOutput, Error};
