use leptos::*;

use rsiot::components::cmp_leptos::components::{tailwind_mwc::Button, Theme};

#[component]
pub fn ThemeSwither() -> impl IntoView {
    let (theme, theme_set) = create_signal("dark".to_string());

    view! {
        <Theme theme=theme.into()/>

        <div class="flex flex-row gap-4">

            <Button
                on_click = move || theme_set.set("light".to_string())
                icon=|| view!{ "" }
                text="light"
            />

            <Button
                on_click = move || theme_set.set("dark".to_string())
                icon=|| view!{ "" }
                text="dark"
            />

            <Button
                on_click = move || theme_set.set("dark-high-contrast".to_string())
                icon=|| view!{ "" }
                text="dark-hc"
            />

            <Button
                on_click = move || theme_set.set("dark-medium-contrast".to_string())
                icon=|| view!{ "" }
                text="dark-mc"
            />

            <Button
                on_click = move || theme_set.set("light-high-contrast".to_string())
                icon=|| view!{ "" }
                text="light-hc"
            />

            <Button
                on_click = move || theme_set.set("light-medium-contrast".to_string())
                icon=|| view!{ "" }
                text="light-mc"
            />

        </div>
    }
}
