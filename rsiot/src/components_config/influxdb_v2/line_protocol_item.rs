use crate::message::*;

use super::ValueType;

/// Строка записи в данных в базу через HTTP API
pub struct LineProtocolItem {
    pub measurement: String,
    pub value: ValueType,
    pub ts: Timestamp,
}

impl LineProtocolItem {
    pub fn new(measurement: &str, value: ValueType, ts: &Timestamp) -> Self {
        Self {
            measurement: measurement.into(),
            value,
            ts: ts.clone(),
        }
    }
}

impl TryFrom<LineProtocolItem> for String {
    type Error = super::Error;

    fn try_from(line_protocol_item: LineProtocolItem) -> Result<Self, Self::Error> {
        (&line_protocol_item).try_into()
    }
}

impl TryFrom<&LineProtocolItem> for String {
    type Error = super::Error;

    fn try_from(line_protocol_item: &LineProtocolItem) -> Result<Self, Self::Error> {
        let measurement = line_protocol_item.measurement.clone();
        let value = match line_protocol_item.value {
            ValueType::bool(value) => value.to_string(),
            ValueType::f64(value) => value.to_string(),
            ValueType::f32(value) => value.to_string(),
            ValueType::i8(value) => value.to_string(),
            ValueType::i16(value) => value.to_string(),
            ValueType::i32(value) => value.to_string(),
            ValueType::u8(value) => value.to_string(),
            ValueType::u16(value) => value.to_string(),
            ValueType::u32(value) => value.to_string(),
        };
        let ts = line_protocol_item
            .ts
            .timestamp_nanos_opt()
            .ok_or(super::Error::WrongTimestamp(line_protocol_item.ts.clone()))?;
        let line = format!("{measurement} value={value} {ts}");
        Ok(line)
    }
}
