/// Ограничение на структуру буфера
pub trait BufferBound
where
    Self: Clone + Default + Send + Sync,
{
}
