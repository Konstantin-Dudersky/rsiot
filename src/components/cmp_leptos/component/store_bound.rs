/// Трейт, необходимо реализовать на структуре
///
/// Структуру можно будет использовать для хранения данных Leptos
///
/// На структуре можно автоматически реализовать:
///
/// ```rust
/// #[derive(Default, Clone, Store)]
/// ```
pub trait StoreBound: Default + Clone + Send + Sync {}
