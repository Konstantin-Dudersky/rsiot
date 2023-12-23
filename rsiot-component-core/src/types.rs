use tokio::sync::{broadcast, mpsc};

pub type Input<TMessage> = broadcast::Receiver<TMessage>;
pub type Output<TMessage> = mpsc::Sender<TMessage>;
