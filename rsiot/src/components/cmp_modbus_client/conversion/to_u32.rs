//! Конвертация двух регистров в u32

use super::utils::{conv_u16x2_to_u8x4, is_wrong_len};

/// Конвертация двух регистров в u32, little endian с пересановкой байт
pub fn little_endian_swap(data: &[u16]) -> u32 {
    if is_wrong_len(data, 2) {
        return u32::default();
    }
    let mut bytes = conv_u16x2_to_u8x4(data);
    bytes.swap(0, 1);
    bytes.swap(2, 3);
    u32::from_le_bytes(bytes)
}
