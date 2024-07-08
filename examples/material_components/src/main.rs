mod app;
mod components;
mod material_components;

use app::*;
use leptos::*;

use rsiot::logging::configure_logging;

fn main() {
    console_error_panic_hook::set_once();

    configure_logging("info").unwrap();

    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
