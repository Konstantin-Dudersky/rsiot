//! Общие данные для компонентов WebSocket

use serde::{de::DeserializeOwned, Serialize};

/// Типаж для сообщение, пересылаемых по вебсокету
///
/// Необходимо реализовать на типе перечисления. Можно реализовать автоматически:
///
/// ```rust
/// #[derive(Clone, Debug, Deserialize, IntoStaticStr, Serialize)]
/// ```
pub trait HttpDataBound:
    Clone + std::fmt::Debug + Default + DeserializeOwned + Send + Serialize + Sync
{
}

impl HttpDataBound for () {}
