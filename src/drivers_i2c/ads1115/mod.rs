//! Коммуникация с АЦП ADS11xx

pub mod config;
mod device;

pub(crate) use device::ADS1115;
