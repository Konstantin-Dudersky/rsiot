use std::fmt::Debug;

pub trait BufferData
where
    Self: Clone + Debug + Default,
{
}
