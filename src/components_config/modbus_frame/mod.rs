mod error;
mod rtu_request;
mod rtu_response;

const CRC_ALG: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_MODBUS);

pub use {
    error::Error,
    rtu_request::{FunctionCode, RTURequest},
    rtu_response::RTUResponse,
};
