//! Доделать или удалить

#[cfg(feature = "cmp_plc")]
mod config_plc;
#[cfg(feature = "cmp_plc")]
mod messages;

#[cfg(feature = "cmp_plc")]
fn main() {
    use messages::*;
}

#[cfg(not(feature = "cmp_plc"))]
fn main() {}
