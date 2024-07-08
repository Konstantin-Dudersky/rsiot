use crate::message::PhyQuantity;

use super::{
    BMP180Data, BMP180Oversampling, CalibrationData, ResponseUncompensatedPressure,
    ResponseUncompensatedTemperature,
};

const POW_2_4: f64 = 16.0;
const POW_2_8: f64 = 256.0;
const POW_2_11: f64 = 2048.0;
const POW_2_12: f64 = 4096.0;
const POW_2_13: f64 = 8192.0;
const POW_2_15: f64 = 32768.0;
const POW_2_16: f64 = 65536.0;

#[allow(non_snake_case)]
pub(super) fn calculate_values(
    cd: &CalibrationData,
    ut: &ResponseUncompensatedTemperature,
    up: &ResponseUncompensatedPressure,
    oversampling: BMP180Oversampling,
) -> BMP180Data {
    // temperature
    let X1 = (ut.UT as f64 - cd.AC6) * cd.AC5 / POW_2_15;
    let X2 = cd.MC * POW_2_11 / (X1 + cd.MD);
    let B5 = X1 + X2;
    let T = (B5 + 8.0) / POW_2_4;

    // pressure
    let B6 = B5 - 4000.0;
    let X1 = (cd.B2 * (B6 * B6 / POW_2_12)) / POW_2_11;
    let X2 = cd.AC2 * B6 / POW_2_11;
    let X3 = X1 + X2;
    let B3 = (((((cd.AC1 * 4.0 + X3) as u64) << (oversampling as u8)) + 2) / 4) as f64;
    let X1 = cd.AC3 * B6 / POW_2_13;
    let X2 = (cd.B1 * (B6 * B6 / POW_2_12)) / POW_2_16;
    let X3 = ((X1 + X2) + 2.0) / 4.0;
    let B4 = cd.AC4 * (X3 + POW_2_15) / POW_2_15;
    let B7 = (up.UP as f64 - B3) * (50000_u64 >> (oversampling as u8)) as f64;
    let p = if B7 < 0x80000000_i64 as f64 {
        (B7 * 2.0) / B4
    } else {
        (B7 / B4) * 2.0
    };
    let X1 = (p / POW_2_8) * (p / POW_2_8);
    let X1 = (X1 * 3038.0) / POW_2_16;
    let X2 = (-7357.0 * p) / POW_2_16;
    let p = p + (X1 + X2 + 3791.0) / POW_2_4;

    // altitude
    let altitude = 44330.0 * (1.0 - (p / 101325.0).powf(1.0 / 5.255));

    BMP180Data {
        temperature: PhyQuantity::new_temperature_C(T / 10.0),
        pressure: PhyQuantity::new_pressure_Pa(p),
        altitude: PhyQuantity::new_length_m(altitude),
    }
}

#[cfg(test)]
mod tests {
    use super::{super::ResponseCalibrationData, *};

    /// Запуск:
    ///
    /// ```bash
    /// cargo test --target x86_64-unknown-linux-gnu --lib --features executor -- drivers_i2c::bmp180::tests::test_calc --exact --show-output
    /// ```
    #[test]
    fn test_calc() {
        let cd = ResponseCalibrationData {
            AC1: 408,
            AC2: -72,
            AC3: -14383,
            AC4: 32741,
            AC5: 32757,
            AC6: 23153,
            B1: 6190,
            B2: 4,
            MB: -32768,
            MC: -8711,
            MD: 2868,
        };
        let cd: CalibrationData = cd.into();
        let ut = ResponseUncompensatedTemperature { UT: 27898 };
        let up = ResponseUncompensatedPressure { UP: 23843 };

        let values = calculate_values(&cd, &ut, &up, BMP180Oversampling::UltraLowPower);
        println!("{values:?}");
    }
}
