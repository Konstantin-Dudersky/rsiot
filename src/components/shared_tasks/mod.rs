//! Задачи, которые можно использовать в других компонентах

pub mod change_mpsc_msg;
pub mod filter_identical_data;
pub mod mpsc_to_msg_bus;
pub mod msg_bus_to_mpsc;

pub use filter_identical_data::FilterIdenticalData;
pub use mpsc_to_msg_bus::MpscToMsgBus;
