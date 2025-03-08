//! Коммуникация с модулем PM-RQ8

mod config;
mod device;
mod error;
mod tasks;

pub use config::Buffer;
pub use config::Config;
pub use device::Device;
pub use error::Error;

type Result<T> = std::result::Result<T, Error>;

// use pm_firmware_lib::pm_rq8_v0_0_3::{I2cRequest, I2cResponse};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum I2cRequest {
    SetOutputs(u8),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum I2cResponse {
    Ok(u32),
}
