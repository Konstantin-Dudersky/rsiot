use leptos::*;

/// Разделитель
///
/// <https://m3.material.io/components/divider/overview>
#[component]
pub fn Divider() -> impl IntoView {
    view! {
        <div class="w-auto border-t border-outline-variant mx-2"></div>
    }
}
