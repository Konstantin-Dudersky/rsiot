use tracing::warn;

/// Конвертация двух чисел u16 в массив 4 байт
pub fn conv_u16x2_to_u8x4(data: &[u16]) -> [u8; 4] {
    let mut bytes = [0; 4];

    let register = data[0].to_be_bytes();
    bytes[0] = register[0];
    bytes[1] = register[1];

    let register = data[1].to_be_bytes();
    bytes[2] = register[0];
    bytes[3] = register[1];
    bytes
}

/// Проверка правильной длины среза
///
/// Если возвращает true - длина неправильная
pub fn is_wrong_len(data: &[u16], need_len: usize) -> bool {
    if data.len() != need_len {
        warn!(
            "Length of slice must be equal to {need_len}, current data: {:?}",
            data,
            need_len = need_len
        );
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_wrong_len() {
        let data = [0, 1, 2, 3, 4];
        assert!(is_wrong_len(&data[0..=1], 0));
        assert!(is_wrong_len(&data[0..=1], 1));
        assert!(!is_wrong_len(&data[0..=1], 2));
        assert!(is_wrong_len(&data[0..=1], 3));
    }
}
