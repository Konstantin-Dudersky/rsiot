use rsiot_messages_core::message_v2::Message;

pub trait DeriveItemProcess<TMsg>: Send + Sync {
    fn process(&mut self, msg: &Message<TMsg>) -> Option<Vec<Message<TMsg>>>;
}
