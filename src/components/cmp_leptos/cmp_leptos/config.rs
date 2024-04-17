use leptos::*;

/// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_leptos.html#config
pub struct Config<TView, TIntoView>
where
    TView: Fn() -> TIntoView,
    TIntoView: IntoView,
{
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_leptos.html#body_component
    pub body_component: TView,

    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_leptos.html#hostname
    pub hostname: String,
}
