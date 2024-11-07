use leptos::prelude::*;

use rsiot::components::cmp_leptos::components::tailwind_mwc::{Card, CardKind};

#[component]
pub fn CardView() -> impl IntoView {
    view! {
        <a href="#">

        <Card
            _kind = CardKind::Outlined
        >
            "Card content"
        </Card>
        </a>
    }
}
