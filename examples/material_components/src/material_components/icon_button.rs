use leptos::prelude::*;
use rsiot::components::cmp_leptos::components::tailwind_mwc::{IconButton, IconButtonKind};
use tracing::info;

#[component]
pub fn IconButtonView() -> impl IntoView {
    view! {
        <div class="grid grid-cols-3 place-items-center">
            <div>
                IconButtonKind
            </div>

            <div>
                Enabled
            </div>

            <div>
                Disabled
            </div>

            <div>
                IconButtonKind::Standard
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::Standard
                    disabled = false.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::Standard
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    disabled = true.into()
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                IconButtonKind::StandardToggle
            </div>

            <div >
                <IconButton
                    kind = IconButtonKind::StandardToggle
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = false.into()
                    on_click = || info!("clicked")
                />

                <IconButton
                    kind = IconButtonKind::StandardToggle
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = true.into()
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::StandardToggle
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = false.into()
                    on_click = || info!("clicked")
                />

                <IconButton
                    kind = IconButtonKind::StandardToggle
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = true.into()
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                IconButtonKind::Filled
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::Filled
                    disabled = false.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::Filled
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                IconButtonKind::FilledToggle
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::FilledToggle
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = false.into()
                    on_click = || info!("clicked")
                />

                <IconButton
                    kind = IconButtonKind::FilledToggle
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = true.into()
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::FilledToggle
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = false.into()
                    on_click = || info!("clicked")
                />

                <IconButton
                    kind = IconButtonKind::FilledToggle
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = true.into()
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                IconButtonKind::FilledTonal
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::FilledTonal
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::FilledTonal
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                IconButtonKind::FilledTonalToggle
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::FilledTonalToggle
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = false.into()
                    on_click = || info!("clicked")
                />

                <IconButton
                    kind = IconButtonKind::FilledTonalToggle
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = true.into()
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::FilledTonalToggle
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = false.into()
                    on_click = || info!("clicked")
                />

                <IconButton
                    kind = IconButtonKind::FilledTonalToggle
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = true.into()
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                IconButtonKind::Outlined
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::Outlined
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::Outlined
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                IconButtonKind::OutlinedToggle
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::OutlinedToggle
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = false.into()
                    on_click = || info!("clicked")
                />

                <IconButton
                    kind = IconButtonKind::OutlinedToggle
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = true.into()
                    on_click = || info!("clicked")
                />
            </div>

            <div>
                <IconButton
                    kind = IconButtonKind::OutlinedToggle
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = false.into()
                    on_click = || info!("clicked")
                />

                <IconButton
                    kind = IconButtonKind::OutlinedToggle
                    disabled = true.into()
                    icon = || view!{ <span class="iconify material-symbols--play-arrow-rounded h-6 w-6"></span> }
                    toggled = true.into()
                    on_click = || info!("clicked")
                />
            </div>

        </div>
    }
}
