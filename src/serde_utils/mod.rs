//! Утилиты для сериализации / десериализации

#![allow(unused_imports)]
#![allow(dead_code)]

#[cfg(feature = "serde_json")]
mod json;
#[cfg(feature = "serde_toml")]
mod toml;

mod error;
// TODO - удалить
pub mod postcard_serde;
mod serde_alg;
mod serde_alg_kind;

pub use error::Error;
pub(crate) use serde_alg::SerdeAlg;
pub use serde_alg_kind::SerdeAlgKind;
