use std::time::Duration;

/// Расчет скорости передачи байт в секунду
///
/// На передачу одного байта приходится больше 8 бит, включая стартовый и стоповый биты.
///
/// TODO: добавить межбайтовое время
pub fn bytes_per_second(
    baudrate: &super::Baudrate,
    data_bits: &super::DataBits,
    stop_bits: &super::StopBits,
) -> f64 {
    let start_bits: f64 = 1.0;
    let data_bits: f64 = data_bits.clone().into();
    let stop_bits: f64 = stop_bits.clone().into();
    let bits_per_byte = start_bits + data_bits + stop_bits;

    let bits_per_second: f64 = baudrate.clone().into();

    bits_per_second / bits_per_byte
}

/// Расчет времени передачи определенного количества байт
pub fn calculate_transmission_time(
    bytes_per_second: f64,
    bytes: usize,
    reserve: Duration,
) -> Duration {
    Duration::from_secs_f64(bytes as f64 / bytes_per_second) + reserve
}
