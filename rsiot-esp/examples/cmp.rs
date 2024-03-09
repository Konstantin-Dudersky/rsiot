fn main() {
    use esp_idf_svc::{log::EspLogger, sys::link_patches};
    use tracing::info;

    link_patches();
    EspLogger::initialize_default();
    info!("esp started");
}
