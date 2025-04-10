use crc::{Crc, CRC_32_ISCSI};

const CRC_ALG: crc::Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
const CRC_LEN: usize = 4;

use super::Error;

pub struct CrcAlg {}

impl CrcAlg {
    /// Рассчитывает CRC для данных и добавляет его в конец
    pub fn calculate(payload: &mut Vec<u8>) {
        let crc = CRC_ALG.checksum(payload).to_be_bytes();
        payload.extend(crc)
    }

    /// Проверяет CRC. Если сумма совпала, то возвращает данные без CRC
    pub fn check(data: &[u8]) -> Result<&[u8], Error> {
        let len = data.len();
        if len <= CRC_LEN {
            return Err(Error::CrcMismatch);
        }

        let payload = &data[..len - CRC_LEN];
        let crc = &data[len - CRC_LEN..];

        let crc_correct = CRC_ALG.checksum(payload).to_be_bytes();

        if crc_correct != crc {
            return Err(Error::CrcMismatch);
        }

        Ok(payload)
    }
}
