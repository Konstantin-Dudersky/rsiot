use std::collections::HashMap;

use crate::message::*;

use super::FieldValue;

/// Строка записи в данных в базу через HTTP API
#[derive(Debug)]
pub struct LineProtocolItem {
    /// Table
    pub table: String,

    /// Словать Тег = Значение тега
    pub tags: HashMap<String, String>,

    /// Словать Поле = Значение поля
    pub fields: HashMap<String, FieldValue>,

    /// Метка времени
    pub ts: Option<Timestamp>,
}

impl LineProtocolItem {
    /// Новая строка записи
    pub fn new_simple(table: &str, value: impl Into<FieldValue>) -> Self {
        Self {
            table: table.into(),
            tags: HashMap::new(),
            fields: HashMap::from([("value".to_string(), value.into())]),
            ts: Some(Timestamp::default()),
        }
    }

    /// Преобразование в строку, для отправки в базу данных
    pub fn to_string(&self) -> Result<String, super::Error> {
        let mut line = String::from("");

        let table = &self.table;
        line.push_str(table);

        let tags = self
            .tags
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<String>>()
            .join(",");
        if !tags.is_empty() {
            line.push(',');
            line.push_str(&tags);
        }

        let fields = self
            .fields
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<String>>()
            .join(",");
        line.push(' ');
        line.push_str(&fields);

        if let Some(ts) = &self.ts {
            let ts = ts
                .timestamp_nanos_opt()
                .ok_or(super::Error::WrongTimestamp(*ts))?;
            line.push(' ');
            line.push_str(&ts.to_string());
        }

        Ok(line)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    /// cargo test --features cmp_influxdb -- components_config::influxdb3::line_protocol_item::tests::test1 --exact --show-output
    #[test]
    fn test1() -> anyhow::Result<()> {
        let lpi = LineProtocolItem {
            table: "cpu".to_string(),
            tags: HashMap::from([
                ("host".to_string(), "Alpha".to_string()),
                ("region".to_string(), "us-west".to_string()),
                ("application".to_string(), "webserver".to_string()),
            ]),
            fields: HashMap::from([
                ("val".to_string(), FieldValue::from(1)),
                ("usage_percent".to_string(), FieldValue::from(20.5)),
                ("status".to_string(), FieldValue::from("OK".to_string())),
            ]),
            ts: None,
        };

        let lpi: String = lpi.to_string()?;

        println!("line protocol: {}", lpi);

        let from_manual = r#"cpu,host=Alpha,region=us-west,application=webserver val=1i,usage_percent=20.5,status="OK""#;
        println!("from manual:   {}", from_manual);

        assert_eq!(
            lpi.chars().sorted().rev().collect::<String>(),
            from_manual.chars().sorted().rev().collect::<String>()
        );

        Ok(())
    }
}
