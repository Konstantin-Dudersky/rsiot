use tokio::sync::mpsc;

pub type StreamInput<TMessage> = Option<mpsc::Receiver<TMessage>>;
pub type StreamOutput<TMessage> = Option<mpsc::Sender<TMessage>>;
