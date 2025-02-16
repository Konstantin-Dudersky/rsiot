//! Значения скорости

/// Значения скорости
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub enum Baudrate {
    #[default]
    _9_600,
    _19_200,
    _38_400,
    _57_600,
    _115_200,
    _230_400,
    _460_800,
    _1_200_000,
    _12_000_000,
}

impl From<Baudrate> for u32 {
    fn from(value: Baudrate) -> Self {
        match value {
            Baudrate::_9_600 => 9_600,
            Baudrate::_19_200 => 19_200,
            Baudrate::_38_400 => 38_400,
            Baudrate::_57_600 => 57_600,
            Baudrate::_115_200 => 115_200,
            Baudrate::_230_400 => 230_400,
            Baudrate::_460_800 => 460_800,
            Baudrate::_1_200_000 => 1_200_000,
            Baudrate::_12_000_000 => 12_000_000,
        }
    }
}

impl From<Baudrate> for f64 {
    fn from(value: Baudrate) -> Self {
        let baudrate_u32: u32 = value.into();
        baudrate_u32 as f64
    }
}
