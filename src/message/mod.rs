//! Представление сообщений в системе.
//!
//! Сообщения представлены типом перечисления (enum). Данные вложены в варианты перечисления.

mod auth_roles;
mod error;
pub mod example_message;
mod example_message_channel;
mod message;
mod message_channel;
mod msg_data;
mod msg_data_bound;
mod msg_key;
mod msg_serde;
mod msg_trace;
mod phy_quantity;
pub mod system_messages;
mod time_to_live;
mod timestamp;

pub use auth_roles::AuthPermissions;
pub use error::Error;
pub use example_message_channel::ExampleMessageChannel;
pub use message::Message;
pub use message_channel::IMessageChannel;
pub use msg_data::MsgData;
pub use msg_data_bound::{MsgDataBound, MsgRoute};
pub use msg_key::MsgKey;
pub use msg_trace::MsgTrace;
pub use phy_quantity::PhyQuantity;
pub use strum::EnumString;
pub use time_to_live::TimeToLiveValue;
pub use timestamp::Timestamp;

pub use serde::{Deserialize, Serialize};
