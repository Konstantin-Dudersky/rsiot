//! Представление сообщений в системе.
//!
//! Сообщения представлены типом перечисления (enum). Данные вложены в варианты перечисления.

mod auth_roles;
mod error;
mod example_message_channel;
mod message;
mod message_channel;
mod msg_data_bound;
mod msg_serde;
mod msg_trace;
mod system_messages;
mod timestamp;

pub mod eav;
pub mod eav_helpers;
pub mod example_message;

pub use auth_roles::AuthRoles;
pub use error::Error;
pub use example_message_channel::ExampleMessageChannel;
pub use message::{Message, MsgType};
pub use message_channel::IMessageChannel;
pub use msg_data_bound::MsgDataBound;
// pub use msg_source::MsgSource;
pub use msg_trace::MsgTrace;
pub use rsiot_macros::message_new;
pub use timestamp::Timestamp;

pub use serde::{Deserialize, Serialize};
