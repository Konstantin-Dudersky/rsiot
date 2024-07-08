//! Обертки над компонентами [Material Web Components](https://material-web.dev)

mod dialog;
mod filled_button;
mod icon_button;
mod switch;
mod text_field;
#[cfg(feature = "cmp_plc")]
mod valve;
#[cfg(feature = "cmp_plc")]
mod valve_analog;

pub use dialog::Dialog;
pub use filled_button::FilledButton;
pub use icon_button::{IconButton, IconButtonKind};
pub use switch::Switch;
pub use text_field::{TextField, TextFieldKind};
#[cfg(feature = "cmp_plc")]
pub use valve::Valve;
#[cfg(feature = "cmp_plc")]
pub use valve_analog::ValveAnalog;
