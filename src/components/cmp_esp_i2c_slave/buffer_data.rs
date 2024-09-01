use std::fmt::Debug;

/// Ограничения на структуру данных буфера
///
/// На структуре необходимо реализовать:
///
/// ```rust
/// #[derive(Clone, Debug, Default)]
/// ```
pub trait BufferData
where
    Self: Clone + Debug + Default + Send,
{
}
