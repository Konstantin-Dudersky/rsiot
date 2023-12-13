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
