use esp_idf_svc::sys::EspError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Deserialization error: {source}")]
    Deserialization {
        #[from]
        source: postcard::Error,
    },

    #[error("Read from ESP: {0}")]
    ReadFromEsp(EspError),

    #[error("Save to ESP: {0}")]
    SaveToEsp(EspError),

    #[error("Send to channel error: {0}")]
    SendChannel(String),

    #[error("Take partition error: {0}")]
    TakePartition(EspError),
}
