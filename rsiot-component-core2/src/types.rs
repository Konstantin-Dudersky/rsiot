use tokio::sync::{broadcast, mpsc};

pub type ComponentInput<TMessage> = broadcast::Receiver<TMessage>;
pub type ComponentOutput<TMessage> = mpsc::Sender<TMessage>;
