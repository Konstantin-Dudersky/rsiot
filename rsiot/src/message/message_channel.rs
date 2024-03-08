use std::fmt::Debug;

/// Трейт для канала сообщений
pub trait IMessageChannel
where
    Self: Clone + Debug + Send,
{
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum MessageChannel {}

    impl IMessageChannel for MessageChannel {}
}
