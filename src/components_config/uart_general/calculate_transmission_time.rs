use std::time::Duration;

/// Расчет скорости передачи с учетом служебных битов
pub fn data_rate(
    baudrate: &super::Baudrate,
    data_bits: &super::DataBits,
    parity: &super::Parity,
    stop_bits: &super::StopBits,
) -> f64 {
    let start_bits: f64 = 1.0;
    let data_bits: f64 = data_bits.clone().into();
    let parity: f64 = parity.clone().into();
    let stop_bits: f64 = stop_bits.clone().into();

    let baudrate: f64 = baudrate.clone().into();

    baudrate * data_bits / (start_bits + data_bits + parity + stop_bits)
}

/// Расчет времени передачи определенного количества байт
pub fn calculate_transmission_time(data_rate: f64, bytes: usize, reserve: Duration) -> Duration {
    Duration::from_secs_f64(bytes as f64 * 8.0 / data_rate) + reserve
}
