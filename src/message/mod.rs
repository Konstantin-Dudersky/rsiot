//! Представление сообщений в системе.
//!
//! Сообщения представлены типом перечисления (enum). Данные вложены в варианты перечисления.

mod auth_roles;
mod error;
pub mod example_message;
mod message;
mod msg_data;
mod msg_data_bound;
mod msg_key;
mod phy_quantity;
pub mod system_messages;
mod timestamp;
mod value_time;

pub use auth_roles::AuthPermissions;
pub use error::Error;
pub use message::Message;
pub use msg_data::MsgData;
pub use msg_data_bound::MsgDataBound;
pub use msg_key::MsgKey;
#[cfg(feature = "deprecated_stend")]
pub use phy_quantity::PhyQuantity;
pub use strum::EnumString;
pub use timestamp::Timestamp;
pub use value_time::ValueTime;

pub use serde::{Deserialize, Serialize};
