use tokio::sync::mpsc;

pub type StreamInput<TMessage> = mpsc::Receiver<TMessage>;
pub type StreamOutput<TMessage> = mpsc::Sender<TMessage>;
