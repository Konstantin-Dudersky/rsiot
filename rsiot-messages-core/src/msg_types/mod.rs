//! Стандартные типы данных, которые могут содержать в себе сообщения.
//!
//! Сообщения могут содержать в себе и другие типы, не перечисленные здесь.

mod command;
mod value;

pub use command::Command;
pub use value::Value;

use chrono::{DateTime, FixedOffset};

type Timestamp = DateTime<FixedOffset>;
