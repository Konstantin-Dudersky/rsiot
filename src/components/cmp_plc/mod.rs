//! Исполнение логики работы в стиле ПЛК.
//!
//! # Примеры
//!
//! ## Создание базового проекта
//!
//! ### `cfg_plc/mod.rs`
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_plc_minimal/cfg_plc/mod.rs")]
//! ```
//!
//! ### `cfg_plc/fn_cycle_init.rs`
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_plc_minimal/cfg_plc/fn_cycle_init.rs")]
//! ```
//!
//! ### `cfg_plc/fn_input.rs`
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_plc_minimal/cfg_plc/fn_input.rs")]
//! ```
//!
//! ### `cfg_plc/fn_output.rs`
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_plc_minimal/cfg_plc/fn_output.rs")]
//! ```
//!
//! ### `cfg_plc/retention.rs`
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_plc_minimal/cfg_plc/retention.rs")]
//! ```
//!
//! ### `cfg_plc/logic/mod.rs`
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_plc_minimal/cfg_plc/logic/mod.rs")]
//! ```
//!
//! ### `cfg_plc/logic/fb_main.rs`
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_plc_minimal/cfg_plc/logic/fb_main.rs")]
//! ```
//!
//! ### `cfg_plc/logic/fb_simple.rs`
//!
//! ```rust
#![doc = include_str!("../../../examples/cmp_plc_minimal/cfg_plc/logic/fb_simple.rs")]
//! ```
//!

mod component;
mod config;
mod error;
mod fn_process;
pub mod plc;
mod tasks;

#[allow(dead_code, unused_imports)]
mod fb_template;
#[allow(dead_code, unused_imports)]
mod fb_template_full;

pub use component::{COMPONENT_NAME, Cmp};
pub use config::{Config, ConfigRetention};
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;
