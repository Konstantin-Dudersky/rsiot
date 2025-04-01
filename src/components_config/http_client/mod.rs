//! Конфигурация http-клиента

mod config;
mod msg_request;
mod msg_response;
mod process_response;
mod request_input;
mod request_kind;
mod request_periodic;

pub use config::Config;
pub(crate) use msg_request::MsgRequest;
pub(crate) use msg_response::MsgResponse;
pub use request_input::{RequestInput, RequestInputConfig};
pub use request_kind::RequestKind;
pub use request_periodic::{RequestPeriodic, RequestPeriodicConfig};

type FnCreateRequest<TMsg, TClientToServer> = fn(&TMsg) -> Option<TClientToServer>;
type FnProcessResponseSuccess<TMsg, TServerToClient> = fn(&TServerToClient) -> Vec<TMsg>;
type FnProcessResponseError<TMsg> = fn() -> Vec<TMsg>;
