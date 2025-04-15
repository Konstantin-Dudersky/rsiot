#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Cannot represent timetamp as Unix time: {0:?}")]
    WrongTimestamp(crate::message::Timestamp),
}
