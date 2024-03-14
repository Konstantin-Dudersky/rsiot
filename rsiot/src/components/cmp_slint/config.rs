use std::sync::Arc;

use slint::Weak;
use slint_interpreter::{ComponentInstance, Value};
use tokio::sync::Mutex;

use crate::message::{Message, MsgDataBound};

#[derive(Clone)]
pub struct Config<TMsg>
where
    Self: Sync,
    TMsg: MsgDataBound,
{
    pub instance: Arc<Mutex<Weak<ComponentInstance>>>,
    pub fn_input: fn(Message<TMsg>) -> Vec<(String, Value)>,
}
 