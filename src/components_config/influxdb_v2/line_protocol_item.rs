use crate::message::*;

use super::ValueType;

/// Строка записи в данных в базу через HTTP API
#[derive(Debug)]
pub struct LineProtocolItem {
    /// measurement
    pub measurement: String,

    /// value
    pub value: ValueType,

    /// Метка времени
    pub ts: Timestamp,
}

impl LineProtocolItem {
    /// Новая строка записи
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
        let measurement = &line_protocol_item.measurement;
        let value = match line_protocol_item.value {
            ValueType::bool(value) => value.to_string(),
            ValueType::f64(value) => value.to_string(),
            ValueType::f32(value) => value.to_string(),
            ValueType::i8(value) => format!("{}i", value),
            ValueType::i16(value) => format!("{}i", value),
            ValueType::i32(value) => format!("{}i", value),
            ValueType::u8(value) => format!("{}u", value),
            ValueType::u16(value) => format!("{}u", value),
            ValueType::u32(value) => format!("{}u", value),
        };
        let ts = line_protocol_item
            .ts
            .timestamp_nanos_opt()
            .ok_or(super::Error::WrongTimestamp(line_protocol_item.ts.clone()))?;
        let line = format!("{measurement} value={value} {ts}");
        Ok(line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// cargo test --target x86_64-unknown-linux-gnu --lib --features cmp_influxdb -- components_config::influxdb_v2::line_protocol_item::tests::test1 --exact --show-output
    #[test]
    fn test1() {
        let value = ValueType::bool(false);
        let ts = Timestamp::default();

        let lpi = LineProtocolItem {
            measurement: "measurement".to_string(),
            value,
            ts,
        };

        let ans: String = lpi.try_into().unwrap();

        println!("{}", ans);
    }
}
