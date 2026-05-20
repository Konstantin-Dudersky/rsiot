use std::{fmt::Debug, time::Duration};

/// Ограничения для структур запросов и ответов
///
/// На структурах необходимо реализовать:
///
/// ```rust
/// #[derive(Clone, Debug)]
/// ```
pub trait RequestResponseBound
where
    Self: Clone + Debug + Send + Sync,
{
    fn request_duration(&self) -> Duration;
}
