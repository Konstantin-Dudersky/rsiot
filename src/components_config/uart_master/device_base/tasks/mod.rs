mod input_request;
mod periodic_request;
mod response;

pub use input_request::InputRequest;
pub use periodic_request::PeriodicRequest;
pub use response::Response;

use super::{Buffer, Result, UartMessage, UartMessageRaw};

type TaskOutput<T> = tokio::sync::mpsc::Sender<T>;
