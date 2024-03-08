#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error when establishing connection: {0}")]
    Connect(gloo::utils::errors::JsError),

    #[error("JoinError: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),

    #[error("fn_input error: {0}")]
    FnInput(anyhow::Error),

    #[error("fn_output error: {0}")]
    FnOutput(anyhow::Error),

    #[error("Message error: {0}")]
    Message(#[from] rsiot_messages_core::Error),

    #[error("Websocker error: {0}")]
    Websocket(#[from] gloo::net::websocket::WebSocketError),

    #[error(transparent)]
    CmpOutput(crate::executor::ComponentError),
}
