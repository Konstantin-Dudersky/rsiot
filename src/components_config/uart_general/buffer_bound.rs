use std::fmt::Debug;

/// Ограничения для буфера данных
///
/// На структурах необходимо реализовать:
///
/// ```rust
/// #[derive(Clone, Debug, Default, PartialEq)]
/// ```
pub trait BufferBound
where
    Self: Clone + Debug + Default + PartialEq + Send + Sync,
{
}
