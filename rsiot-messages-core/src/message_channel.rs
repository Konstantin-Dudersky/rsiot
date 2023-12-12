/// Трейт для канала сообщений
pub trait IMessageChannel {}

mod tests {
    use super::*;

    pub enum MessageChannel {}

    impl IMessageChannel for MessageChannel {}
}
