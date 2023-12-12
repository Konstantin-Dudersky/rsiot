#![doc = include_str!("../README.md")]

mod error;
mod example_message;
mod imessage;
mod message_channel;

pub mod eav;
pub mod eav_helpers;
pub mod msg_types;

pub use error::Error;
pub use example_message::ExampleMessage;
pub use imessage::IMessage;
pub use message_channel::IMessageChannel;
pub use serde::{Deserialize, Serialize};
