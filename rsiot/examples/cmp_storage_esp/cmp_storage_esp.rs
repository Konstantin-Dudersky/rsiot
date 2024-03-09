//! cargo run -p rsiot --example cmp_storage_esp --target="riscv32imc-esp-espidf" --features="cmp_storage_esp, single-thread"

#[cfg(feature = "cmp_storage_esp")]
fn main() {
    use esp_idf_svc::{log::EspLogger, sys::link_patches};
    use tracing::info;

    link_patches();
    EspLogger::initialize_default();
    info!("esp started");
}

#[cfg(not(feature = "cmp_storage_esp"))]
fn main() {}
