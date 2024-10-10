use leptos::*;

/// Конфигурация компонента cmp_leptos
pub struct Config<TView, TIntoView>
where
    TView: Fn() -> TIntoView,
    TIntoView: IntoView,
{
    /// Корневой компонент для монтирования
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// body_component: || view! { <App/> }
    /// ```
    pub body_component: TView,

    /// Имя хоста, на котором развернуто веб-приложение
    pub hostname: String,
}
