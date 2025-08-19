//! Общие данные для компонентов WebSocket

use serde::{de::DeserializeOwned, Serialize};

// ANCHOR: WebsocketMessage
/// Типаж для сообщение, пересылаемых по вебсокету
///
/// Необходимо реализовать на типе перечисления. Можно реализовать автоматически:
///
/// ```rust
/// #[derive(Clone, Debug, Deserialize, IntoStaticStr, Serialize)]
/// ```
pub trait WebsocketMessage:
    Clone + std::fmt::Debug + DeserializeOwned + Into<&'static str> + Send + Serialize + Sync
{
}
// ANCHOR: WebsocketMessage
