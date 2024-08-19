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
mod msg_serde;
mod msg_trace;
mod phy_quantity;
mod service;
pub mod system_messages;
mod time_to_live;
mod timestamp;

/// Пока эти модули не нужны, в будущем скорее всего можно удалить
pub mod eav;
#[allow(unused_imports)]
mod eav_helpers;

pub use auth_roles::AuthPermissions;
pub use error::Error;
pub use example_message_channel::ExampleMessageChannel;
pub use message::Message;
pub use message_channel::IMessageChannel;
pub use msg_data::MsgData;
pub use msg_data_bound::MsgDataBound;
pub use msg_trace::MsgTrace;
pub use phy_quantity::PhyQuantity;
pub use rsiot_macros::message_new;
pub use service::ServiceBound;
pub use strum::EnumString;
pub use time_to_live::{TimeToLive, TimeToLiveValue};
pub use timestamp::Timestamp;

pub use serde::{Deserialize, Serialize};
