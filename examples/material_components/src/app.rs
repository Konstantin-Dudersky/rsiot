use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};

use crate::{leptos_components::ThemeSwither, material_components};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>

            <nav>
                <A href="/button"> Button </A>
                <A href="/card"> Card </A>
                <A href="/divider"> Divider </A>
                <A href="/icon_button"> IconButton </A>
                <A href="/text_field"> TextField </A>
                <A href="/drives"> Drives </A>
            </nav>

            <main class="container flex flex-col mx-auto">
                <ThemeSwither />

                <Routes fallback=move || "Page not found">
                    <Route path=path!("/button") view=material_components::ButtonView/>
                    <Route path=path!("/card") view=material_components::CardView/>
                    <Route path=path!("/divider") view=material_components::DividerView/>
                    <Route path=path!("/icon_button") view=material_components::IconButtonView/>
                    <Route path=path!("/text_field") view=material_components::TextFieldView/>
                    <Route path=path!("/drives") view=material_components::Drives/>
                </Routes>

            </main>

        </Router>
    }
}
