//! Общие задачи для работы HTTP-клиента

mod error;
mod http_client_general;
mod tasks;

pub use error::Error;
pub use http_client_general::HttpClientGeneral;

type Result<T> = std::result::Result<T, Error>;
