use std::fmt::Debug;

/// Ограничения для буфера данных
///
/// На структурах необходимо реализовать:
///
/// ```rust
/// #[derive(Clone, Debug, Default)]
/// ```
pub trait BufferBound
where
    Self: Clone + Debug + Default + Send + Sync,
{
}

impl BufferBound for () {}
