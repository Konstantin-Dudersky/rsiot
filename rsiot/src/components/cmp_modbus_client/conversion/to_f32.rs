//! Конвертация двух регистров в f32

use super::utils::{conv_u16x2_to_u8x4, is_wrong_len};

/// Конвертация двух регистров в f32, little endian
pub fn little_endian(_data: &[u16]) -> f32 {
    todo!()
}

/// Конвертация двух регистров в f32, little endian с перестановкой байт
pub fn little_endian_swap(data: &[u16]) -> f32 {
    if is_wrong_len(data, 2) {
        return f32::default();
    }
    let mut bytes = conv_u16x2_to_u8x4(data);
    bytes.swap(0, 1);
    bytes.swap(2, 3);
    f32::from_le_bytes(bytes)
}

/// Конвертация двух регистров в f32, big endian с перестановкой байт
pub fn big_endian(_data: &[u16]) -> f32 {
    todo!()
}

/// Конвертация двух регистров в f32, big endian с перестановкой байт
pub fn big_endian_swap(_data: &[u16]) -> f32 {
    todo!()
}
