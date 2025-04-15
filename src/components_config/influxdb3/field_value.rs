use std::fmt::Display;

/// Значение в словаре `fields`
#[derive(Clone, Debug)]
pub struct FieldValue(String);

impl From<bool> for FieldValue {
    fn from(value: bool) -> Self {
        FieldValue(value.to_string())
    }
}

impl From<f32> for FieldValue {
    fn from(value: f32) -> Self {
        FieldValue(value.to_string())
    }
}

impl From<f64> for FieldValue {
    fn from(value: f64) -> Self {
        FieldValue(value.to_string())
    }
}

impl From<i8> for FieldValue {
    fn from(value: i8) -> Self {
        FieldValue(format!("{}i", value))
    }
}

impl From<i16> for FieldValue {
    fn from(value: i16) -> Self {
        FieldValue(format!("{}i", value))
    }
}

impl From<i32> for FieldValue {
    fn from(value: i32) -> Self {
        FieldValue(format!("{}i", value))
    }
}

impl From<u8> for FieldValue {
    fn from(value: u8) -> Self {
        FieldValue(format!("{}u", value))
    }
}

impl From<u16> for FieldValue {
    fn from(value: u16) -> Self {
        FieldValue(format!("{}u", value))
    }
}

impl From<u32> for FieldValue {
    fn from(value: u32) -> Self {
        FieldValue(format!("{}u", value))
    }
}

impl From<String> for FieldValue {
    fn from(value: String) -> Self {
        FieldValue(format!("\"{}\"", value))
    }
}

impl Display for FieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
