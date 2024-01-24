#![doc = include_str!("../README.md")]

mod error;
mod example_message;
mod example_message_channel;
mod imessage;
mod message_channel;
mod msg_content;

pub mod eav;
pub mod eav_helpers;
pub mod msg_meta;
pub mod msg_types;

pub use error::Error;
pub use example_message::ExampleMessage;
pub use example_message_channel::ExampleMessageChannel;
pub use imessage::IMessage;
pub use message_channel::IMessageChannel;
pub use msg_content::MsgContent;
pub use msg_meta::MsgMeta;
pub use rsiot_macros::MsgMeta;
pub use serde::{Deserialize, Serialize};
