#[cfg(feature = "cmp_plc")]
mod plc_drives_shared;

#[cfg(feature = "cmp_plc")]
mod plc_drives_motor;
#[cfg(feature = "cmp_plc")]
pub use plc_drives_motor::plc_drives_motor;

#[cfg(feature = "cmp_plc")]
mod plc_drives_valve_analog;
#[cfg(feature = "cmp_plc")]
pub use plc_drives_valve_analog::plc_drives_valve_analog;

#[cfg(feature = "cmp_plc")]
mod plc_drives_valve;
#[cfg(feature = "cmp_plc")]
pub use plc_drives_valve::plc_drives_valve;

mod text_content;
pub use text_content::text_content;
