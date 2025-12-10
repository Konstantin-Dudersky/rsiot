use bytes::{Buf, Bytes, BytesMut};

use super::{CRC_ALG, Error};

pub struct RTUResponse {
    pub address: u8,
    pub data: Bytes,
}

impl RTUResponse {
    pub fn parse(self) -> Result<BytesMut, Error> {
        let mut frame: BytesMut = self.data.into();

        let mut crc_packet = frame.split_off(frame.len() - 2);
        let crc_packet: u16 = crc_packet.get_u16_le();
        let crc_calculated = CRC_ALG.checksum(&frame);
        if crc_packet != crc_calculated {
            return Err(Error::CRCMismatch);
        }

        let address = frame.try_get_u8()?;
        if address != self.address {
            return Err(Error::AddressMismatch {
                address_in_packet: address,
                expected_address: self.address,
            });
        }

        let _func_code = frame.try_get_u8()?;

        let length = frame.try_get_u8()? as usize;
        if length != frame.len() {
            return Err(Error::LengthMismatch {
                length_in_packet: length,
                expected_length: frame.len(),
            });
        }

        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test() -> anyhow::Result<()> {
        let data = vec![
            0x01, 0x03, 0x16, 0x03, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x01, 0x39, 0x42, 0x38,
        ];

        let correct = Bytes::from(data[3..25].to_vec());

        let test = RTUResponse {
            address: 1,
            data: Bytes::from_owner(data),
        }
        .parse()?;

        assert_eq!(test, correct);

        Ok(())
    }
}
