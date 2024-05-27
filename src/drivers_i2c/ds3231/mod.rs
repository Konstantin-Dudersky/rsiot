//! Подключение часов реального времени DS3231

mod data_models;
mod device;
mod task_input;
mod task_output;

pub use device::DS3231;
pub use task_input::InputData;
pub use task_output::OutputData;
