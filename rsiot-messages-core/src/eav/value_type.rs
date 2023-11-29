#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum ValueType {
    bool(bool),
    f64(f64),
    String(String),
    u64(u64),
}

impl From<bool> for ValueType {
    fn from(value: bool) -> Self {
        ValueType::bool(value)
    }
}

impl From<f64> for ValueType {
    fn from(value: f64) -> Self {
        ValueType::f64(value)
    }
}

impl From<String> for ValueType {
    fn from(value: String) -> Self {
        ValueType::String(value)
    }
}

impl From<u64> for ValueType {
    fn from(value: u64) -> Self {
        ValueType::u64(value)
    }
}

impl From<ValueType> for u64 {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::bool(_) => todo!(),
            ValueType::f64(value) => value as u64,
            ValueType::String(_) => todo!(),
            ValueType::u64(value) => value,
        }
    }
}

impl Default for ValueType {
    fn default() -> Self {
        Self::f64(0.0)
    }
}
