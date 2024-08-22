use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

use super::{ServiceBound, TimeToLiveValue};

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
    Clone + Debug + DeserializeOwned + PartialEq + Send + Serialize + Sync
{
    /// Перечисление, содержащее названия всех сервисов
    type TService: ServiceBound;

    /// Разрешенные маршруты сообщения
    fn define_enabled_routes(&self) -> Vec<(Option<Self::TService>, Option<Self::TService>)> {
        vec![(None, None)]
    }

    /// Задать ограничение времени жизни сообщения
    ///
    /// # Примеры
    ///
    /// Все сообщения без ограничения по времени
    ///
    /// ```rust
    /// impl MsgDataBound for Custom {
    ///     fn define_time_to_live(&self) -> TimeToLiveValue {
    ///         TimeToLiveValue::Infinite
    ///     }
    /// }
    /// ```
    fn define_time_to_live(&self) -> TimeToLiveValue {
        TimeToLiveValue::Infinite
    }
}
