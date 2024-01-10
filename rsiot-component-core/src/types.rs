use tokio::sync::{broadcast, mpsc};

use crate::ComponentError;

pub type ComponentInput<TMessage> = broadcast::Receiver<TMessage>;
pub type ComponentOutput<TMessage> = mpsc::Sender<TMessage>;
pub type ComponentResult = Result<(), ComponentError>;
