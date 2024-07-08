mod device;
mod pcf8575_pin_mode;
mod state;
mod task_read_inputs;
mod task_write_output;

pub use device::PCF8575;
pub use pcf8575_pin_mode::PCF8575PinMode;

use crate::message::Message;

type TPinFnOutput<TMsg> = Vec<(usize, fn(bool) -> Option<Message<TMsg>>)>;
