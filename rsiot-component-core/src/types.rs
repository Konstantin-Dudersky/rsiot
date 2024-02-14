use tokio::sync::broadcast;

use crate::ComponentError;

pub type ComponentInput<TMessage> = broadcast::Receiver<TMessage>;
pub type ComponentResult = Result<(), ComponentError>;
