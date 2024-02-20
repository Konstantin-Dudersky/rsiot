mod message;
mod msg_data_bound;
mod msg_serde;
mod msg_source;

pub use message::{Message, MsgContentType};
pub use msg_data_bound::MsgContentBound;
pub use msg_source::MsgSource;
