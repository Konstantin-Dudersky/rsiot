use rsiot_messages_core::{
    eav::{EavModel, ValueType},
    message_v2::Message,
    msg_meta::Timestamp,
};

#[derive(Clone, Debug)]
pub struct Config<TMsg> {
    pub host: String,
    pub port: u16,
    pub org: String,
    pub bucket: String,
    pub token: String,

    /// Функция преобразования сообщения в строки протокола InfluxDB
    ///
    /// **Примеры**
    ///
    /// Как общий случай можно использовать представление EAV. Тогда функция будет выглядить так:
    ///
    /// ```rust
    /// |msg: &TMsg| cmp_influxdb::msg_into_line_protocol(msg)
    /// ```
    pub fn_input: fn(&Message<TMsg>) -> Vec<LineProtocolItem>,
}

/// Строка записи в данных в базу через HTTP API
pub struct LineProtocolItem {
    pub measurement: String,
    pub value: DataPointVaueType,
    pub ts: Timestamp,
}

impl TryFrom<LineProtocolItem> for String {
    type Error = crate::Error;

    fn try_from(line_protocol_item: LineProtocolItem) -> Result<Self, Self::Error> {
        (&line_protocol_item).try_into()
    }
}

impl TryFrom<&LineProtocolItem> for String {
    type Error = crate::Error;

    fn try_from(line_protocol_item: &LineProtocolItem) -> Result<Self, Self::Error> {
        let measurement = line_protocol_item.measurement.clone();
        let value = match line_protocol_item.value {
            crate::DataPointVaueType::f64(value) => value.to_string(),
        };
        let ts = line_protocol_item
            .ts
            .timestamp_nanos_opt()
            .ok_or(crate::Error::WrongTimestamp(line_protocol_item.ts.clone()))?;
        let line = format!("{measurement} value={value} {ts}");
        Ok(line)
    }
}

#[allow(non_camel_case_types)]
pub enum DataPointVaueType {
    f64(f64),
}

impl From<ValueType> for DataPointVaueType {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::bool(_) => todo!(),
            ValueType::f64(value) => Self::f64(value),
            ValueType::String(_) => todo!(),
            ValueType::u64(_) => todo!(),
        }
    }
}

/// Преобразование модели данных EAV в строку протокола InfluxDB
fn eav_to_line_protocol(eav: &EavModel) -> LineProtocolItem {
    LineProtocolItem {
        measurement: eav.entity.clone(),
        value: eav.value.clone().into(),
        ts: eav.ts.clone(),
    }
}

/// Преобразование сообщения в вектор строк протокола InfluxDB
pub fn msg_into_line_protocol<TMsg>(msg: &TMsg) -> Vec<LineProtocolItem> {
    // msg.clone()
    //     .into_eav()
    //     .iter()
    //     .map(eav_to_line_protocol)
    //     .collect()
    // TODO
    vec![]
}
