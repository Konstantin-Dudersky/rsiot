use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use super::TimeToLive;

/// Ограничения на данные, которые могут содержать сообщения
///
/// На перечислениях можно автоматически реализовывать трейты:
///
/// ```rust
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
/// ```
pub trait MsgDataBound:
    Clone + Debug + DeserializeOwned + PartialEq + Send + Serialize + Sync + TimeToLive
{
    type TService;

    /// Разрешен ли марштур данного сообщения
    /// TODO - убрать реализацию по-умолчанию
    fn is_route_enabled(&self, src: Option<Self::TService>, dst: Option<Self::TService>) -> bool {
        let _ = src;
        let _ = dst;
        true
    }
}
