use leptos::*;

pub struct Config<TView, TIntoView>
where
    TView: Fn() -> TIntoView,
    TIntoView: IntoView,
{
    /// Корневой компонент для монтирования
    ///
    /// Пример:
    /// ```rust
    /// || view! { <App/> }
    /// ```
    pub body_component: TView,

    pub hostname: String,
}
