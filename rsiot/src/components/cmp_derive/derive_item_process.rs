use crate::message::Message;

pub trait DeriveItemProcess<TMsg>: Send + Sync {
    fn process(&mut self, msg: &Message<TMsg>) -> Option<Vec<Message<TMsg>>>;
}
