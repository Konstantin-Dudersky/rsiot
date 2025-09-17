use crate::executor::ComponentError;

use super::COMPONENT_NAME;

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} AttributeAccess: {0}")]
    AttributeAccess(#[from] quick_xml::events::attributes::AttrError),

    #[error("{COMPONENT_NAME} TaskEnd")]
    TaskEnd,

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{COMPONENT_NAME} | TokioTaskJoin: {0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("{COMPONENT_NAME} | VecToString: {0}")]
    VecToString(#[from] std::string::FromUtf8Error),

    #[error("{COMPONENT_NAME} WriteEvent: {0}")]
    WriteEvent(std::io::Error),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
