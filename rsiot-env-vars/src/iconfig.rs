//! Трейт, который должна реализовывать структура конфигурации

use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};

/// Трейт, который должна реализовывать структура конфигурации
pub trait IConfig
where
    Self: Clone + Debug + Default + DeserializeOwned + Serialize,
{
}
