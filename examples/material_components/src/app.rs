use leptos::*;
use leptos_router::*;

use crate::{components::ThemeSwither, material_components};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>

            <nav>
                <A href="/icon_button"> IconButton </A>
                <A href="/button"> Button </A>
                <A href="/drives"> Drives </A>
            </nav>

            <main class="container flex flex-col mx-auto">
                <ThemeSwither />

                <Routes>
                    <Route path="/icon_button" view=material_components::IconButtonView/>
                    <Route path="/button" view=material_components::ButtonView/>
                    <Route path="/drives" view=material_components::Drives/>
                </Routes>

            </main>

        </Router>
    }
}
