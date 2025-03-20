//! Запуск общих задач компонентов cmp_websocket*

mod error;
mod tasks;
mod websocket_client_general_tasks;

pub use error::Error;
pub use tasks::ConnectionState;
pub use websocket_client_general_tasks::WebsocketClientGeneralTasks;

/// Тип Result с заданным типом ошибки
pub type Result<T> = std::result::Result<T, Error>;
