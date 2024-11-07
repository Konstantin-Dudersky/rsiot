use leptos::prelude::*;

/// Вид отображения карточки
#[allow(missing_docs)]
pub enum CardKind {
    Elevated,
    Filled,
    Outlined,
}

#[component]
pub fn Card(
    /// Вид отображения карточки
    #[prop(default = CardKind::Outlined)]
    _kind: CardKind,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="px-4 rounded-xl border bg-surface border-outline-variant">
            { children() }
        </div>
    }
}
