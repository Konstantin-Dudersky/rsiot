//! Типы данных для конфигурации устройства ADS1115

use std::time::Duration;

use bitvec::prelude::*;

use crate::message::Message;

/// Конфигурация входного значения
#[derive(Clone)]
pub struct InputConfig<TMsg> {
    /// Выбор входа для измерения
    pub mux_config: MuxConfig,
    /// Диапазон измерения
    pub amplifier: Amplifier,
    /// Периодичность вызова
    pub period: Duration,
    /// Функция преобразования измеренного значения в вольтах, в исходящее сообщение
    pub fn_output: fn(f64) -> Option<Message<TMsg>>,
}

/// Выбор входа для измерения
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum MuxConfig {
    /// AINp = AIN0 and AINn = GND
    Single_0,
    /// AINp = AIN1 and AINn = GND
    Single_1,
    /// AINp = AIN2 and AINn = GND
    Single_2,
    /// AINp = AIN3 and AINn = GND
    Single_3,
    /// AINp = AIN0 and AINn = AIN1
    Diff_0_1,
    /// AINp = AIN0 and AINn = AIN3
    Diff_0_3,
    /// AINp = AIN1 and AINn = AIN3
    Diff_1_3,
    /// AINp = AIN2 and AINn = AIN3
    Diff_2_3,
}

/// Диапазон измерения. Задает максимальное значение для масштабирования. Напряжение на входе не
/// должно превышать Vdd + 0,3В!
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub enum Amplifier {
    /// ±6.144V
    V_6_144,
    /// ±4.096V
    V_4_096,
    /// ±2.048V
    V_2_048,
    /// ±1.024V
    V_1_024,
    /// ±0.512V
    V_0_512,
    /// ±0.256V
    V_0_256,
}

impl Amplifier {
    /// Максимальное значение преобразования АЦП
    pub fn max_value(&self) -> f64 {
        match self {
            Amplifier::V_6_144 => 6.144,
            Amplifier::V_4_096 => 4.096,
            Amplifier::V_2_048 => 2.048,
            Amplifier::V_1_024 => 1.024,
            Amplifier::V_0_512 => 0.512,
            Amplifier::V_0_256 => 0.256,
        }
    }
}

/// Преобразование конфигурации в байты для передачи в ADS1115
pub fn config_to_bytes(mux_config: &MuxConfig, aplifier: &Amplifier) -> [u8; 2] {
    let mut config: [u8; 2] = [0, 0];
    let config_bits = config.view_bits_mut::<Lsb0>();

    // OS: begin single conversion
    config_bits.set(15, true);

    // MUX: input multiplexer configuration
    match mux_config {
        MuxConfig::Diff_0_1 => {
            config_bits.set(14, false);
            config_bits.set(13, false);
            config_bits.set(12, false);
        }
        MuxConfig::Diff_0_3 => {
            config_bits.set(14, false);
            config_bits.set(13, false);
            config_bits.set(12, true);
        }
        MuxConfig::Diff_1_3 => {
            config_bits.set(14, false);
            config_bits.set(13, true);
            config_bits.set(12, false);
        }
        MuxConfig::Diff_2_3 => {
            config_bits.set(14, false);
            config_bits.set(13, true);
            config_bits.set(12, true);
        }
        MuxConfig::Single_0 => {
            config_bits.set(14, true);
            config_bits.set(13, false);
            config_bits.set(12, false);
        }
        MuxConfig::Single_1 => {
            config_bits.set(14, true);
            config_bits.set(13, false);
            config_bits.set(12, true);
        }
        MuxConfig::Single_2 => {
            config_bits.set(14, true);
            config_bits.set(13, true);
            config_bits.set(12, false);
        }
        MuxConfig::Single_3 => {
            config_bits.set(14, true);
            config_bits.set(13, true);
            config_bits.set(12, true);
        }
    }

    // PGA: programmable gain amplifier configuration
    match aplifier {
        Amplifier::V_6_144 => {
            config_bits.set(11, false);
            config_bits.set(10, false);
            config_bits.set(9, false);
        }
        Amplifier::V_4_096 => {
            config_bits.set(11, false);
            config_bits.set(10, false);
            config_bits.set(9, true);
        }
        Amplifier::V_2_048 => {
            config_bits.set(11, false);
            config_bits.set(10, true);
            config_bits.set(9, false);
        }
        Amplifier::V_1_024 => {
            config_bits.set(11, false);
            config_bits.set(10, true);
            config_bits.set(9, true);
        }
        Amplifier::V_0_512 => {
            config_bits.set(11, true);
            config_bits.set(10, false);
            config_bits.set(9, false);
        }
        Amplifier::V_0_256 => {
            config_bits.set(11, true);
            config_bits.set(10, false);
            config_bits.set(9, true);
        }
    }

    // MODE: device operating mode
    config_bits.set(8, true);

    // DR: data rate
    config_bits.set(7, true);
    config_bits.set(6, false);
    config_bits.set(5, false);

    // COMP_MODE
    config_bits.set(4, false);

    // COMP_POLE
    config_bits.set(3, false);

    // COMP_LAT
    config_bits.set(2, false);

    // COMP_QUE
    config_bits.set(1, true);
    config_bits.set(0, true);

    config.swap(0, 1);
    config
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Запуск:
    ///
    /// ```bash
    /// cargo test --target x86_64-unknown-linux-gnu --lib --features executor --features cmp_esp -- drivers_i2c::ads1115::config::tests::test1 --exact --show-output
    /// ```
    #[test]
    fn test1() {
        let config = config_to_bytes(&MuxConfig::Diff_0_1, &Amplifier::V_2_048);
        assert_eq!(config, [0x85, 0x83]);
    }
}
