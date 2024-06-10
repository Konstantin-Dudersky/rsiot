//! Обертки над компонентами [Material Web Components](https://material-web.dev)

mod dialog;
mod filled_button;
mod icon_button;
mod motor;
mod switch;
mod text_field;
mod valve;
mod valve_analog;

pub use dialog::Dialog;
pub use filled_button::FilledButton;
pub use icon_button::{IconButton, IconButtonKind};
pub use motor::Motor;
pub use switch::Switch;
pub use text_field::{TextField, TextFieldKind};
pub use valve::Valve;
pub use valve_analog::ValveAnalog;
