//! Компоненты для использования в проектах leptos

mod go2rtc;
mod material_theme;
pub mod mwc;
pub mod svg_dynamic;
pub mod tailwind;
pub mod tailwind_mwc;
mod theme;

#[cfg(feature = "cmp_plc")]
pub mod plc_lib;

pub use go2rtc::Go2rtc;
pub use material_theme::MaterialTheme;
pub use theme::Theme;
