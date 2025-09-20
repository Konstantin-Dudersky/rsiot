use crate::executor::ComponentError;

use super::COMPONENT_NAME;

/// Ошибки cmp_esp_wifi
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{COMPONENT_NAME} | CreateAsyncWifi: {0}")]
    CreateAsyncWifi(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | CreateEspWifi: {0}")]
    CreateEspWifi(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | HeaplessString: {0}")]
    HeaplessString(String),

    #[error("{COMPONENT_NAME} | SetConfiguration: {0}")]
    SetConfiguration(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | TokioSyncMpscSend")]
    TokioSyncMpscSend,

    #[error("{COMPONENT_NAME} | WaitNetifUp: {0}")]
    WaitNetifUp(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | WiFiIsConnected: {0}")]
    WiFiIsConnected(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | WiFiStart: {0}")]
    WiFiStart(esp_idf_svc::sys::EspError),

    #[error("{COMPONENT_NAME} | WiFiDisconnect: {0}")]
    WiFiDisconnect(esp_idf_svc::sys::EspError),
}

impl From<Error> for ComponentError {
    fn from(value: Error) -> Self {
        ComponentError::Execution(value.to_string())
    }
}
