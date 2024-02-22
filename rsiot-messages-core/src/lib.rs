//! Представление сообщений в системе.
//!
//! Сообщения представлены типом перечисления (enum). Данные вложены в варианты перечисления.

mod auth_roles;
mod error;
pub mod example_message;
mod example_message_channel;
// mod imessage;
mod message_channel;
mod message_v2;
mod msg_content;
mod msg_content_value;

pub mod eav;
pub mod eav_helpers;
mod msg_meta;

pub use auth_roles::AuthRoles;
pub use error::Error;
pub use example_message_channel::ExampleMessageChannel;
// pub use imessage::IMessage;
pub use message_channel::IMessageChannel;
pub use message_v2::{Message, MsgDataBound, MsgSource, MsgType};
pub use msg_content::MsgContent;
pub use msg_content_value::IMsgContentValue;
pub use msg_meta::Timestamp;
pub use serde::{Deserialize, Serialize};
