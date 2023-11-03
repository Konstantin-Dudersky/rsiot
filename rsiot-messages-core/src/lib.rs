pub trait IMessage
where
    Self: std::fmt::Debug + Send,
{
}
