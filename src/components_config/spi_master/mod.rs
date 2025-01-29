//! Структуры данных для реализации мастера шины SPI

mod request;
mod response;

pub use request::{Operation, Request};
pub use response::Response;
