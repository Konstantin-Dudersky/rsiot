use crate::components::shared_tasks;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Error bind to port: {0}")]
    BindToPort(std::io::Error),

    #[error("{0}")]
    TokioTaskJoin(#[from] tokio::task::JoinError),

    #[error("TokioSyncMpsc")]
    TokioSyncMpsc,

    #[error("{0}")]
    FnInput(anyhow::Error),

    #[error("Error: {err}, text from client: {data}")]
    FnOutput { err: anyhow::Error, data: String },

    #[error("Client disconnected")]
    ClientDisconnected,

    #[error(transparent)]
    CmpOutput(crate::executor::ComponentError),

    #[error("TaskEndInput")]
    TaskEndInput,

    #[error("TaskEndOutput")]
    TaskEndOutput,

    #[error(transparent)]
    SharedTaskMsgBusToMpsc(shared_tasks::msgbus_to_mpsc::Error),

    #[error(transparent)]
    SharedTaskMpscToMsgBus(shared_tasks::mpsc_to_msgbus::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
