use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

/// Ограничения для структур запросов и ответов
///
/// На структурах необходимо реализовать:
///
/// ```rust
/// #[derive(Clone, Debug, Deserialize, Serialize)]
/// ```
pub trait RequestResponseBound
where
    Self: Clone + Debug + DeserializeOwned + Send + Serialize + Sync,
{
}
