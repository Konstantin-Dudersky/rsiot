//! Общие данные для компонентов WebSocket

use serde::{de::DeserializeOwned, Serialize};

/// Типаж для сообщение, пересылаемых по вебсокету
///
/// Необходимо реализовать на типе перечисления. Необходимо реализовать автоматически:
///
/// ```rust
/// #[derive(Clone, Debug, Deserialize, IntoStaticStr, Serialize)]
/// ```
pub trait WebsocketMessage:
    Clone + std::fmt::Debug + DeserializeOwned + Into<&'static str> + Send + Serialize + Sync
{
}
