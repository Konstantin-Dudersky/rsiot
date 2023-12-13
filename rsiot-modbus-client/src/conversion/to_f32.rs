use tracing::warn;

use super::utils::conv_u16x2_to_u8x4;

pub fn little_endian(_data: &[u16]) -> f32 {
    todo!()
}

/// Конвертация двух чисел u16 в f32, little endian с перестановкой байт
pub fn little_endian_swap(data: &[u16]) -> f32 {
    if data.len() != 2 {
        warn!(
            "Length of slice must be equal to 2, current data: {:?}",
            data
        );
        return f32::default();
    }
    let mut bytes = conv_u16x2_to_u8x4(data);
    bytes.swap(0, 1);
    bytes.swap(2, 3);
    f32::from_le_bytes(bytes)
}

pub fn big_endian(_data: &[u16]) -> f32 {
    todo!()
}

pub fn big_endian_swap(_data: &[u16]) -> f32 {
    todo!()
}
