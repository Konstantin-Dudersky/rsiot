use std::fmt::Debug;

use serde::{Serialize, de::DeserializeOwned};

use crate::message::Message;

use super::MsgKey;

/// Ограничения на данные, которые могут содержать сообщения
///
/// На перечислениях можно автоматически реализовывать трейты:
///
/// ```rust
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, Deserialize, MsgKey, PartialEq, Serialize)]
/// ```
pub trait MsgDataBound:
    Clone + Debug + DeserializeOwned + MsgKey + PartialEq + Send + Serialize + Sync
{
    /// Преобразовать данные в сообщение MsgData::Custom
    fn to_message(self) -> Message<Self> {
        Message::new_custom(self)
    }
}
