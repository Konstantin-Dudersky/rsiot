use std::fmt::Debug;

pub trait IMessage
where
    Self: Clone + Debug + Send,
{
}
