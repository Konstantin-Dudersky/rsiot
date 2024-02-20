mod msg_serde;
mod msg_source;

pub use msg_source::MsgSource;

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::msg_meta::Timestamp;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SystemMsg {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MsgType<TData> {
    System(SystemMsg),
    Data(TData),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message<TData> {
    pub content: MsgType<TData>,
    pub key: String,
    pub ts: Timestamp,
    pub source: Option<MsgSource>,
    pub process: Option<MsgSource>,
}

impl<TData> Message<TData>
where
    TData: Debug + Serialize,
{
    pub fn new(msg: TData) -> Self {
        let content = MsgType::Data(msg);
        let full_str = format!("{:?}", content);
        let key = full_str.split("(").into_iter().collect::<Vec<&str>>();
        // Убираем последний элемент. Если тип unit (), нужно убрать два последних элемента
        let skip = if key[key.len() - 2] == "" { 2 } else { 1 };
        let key = key[0..key.len() - skip].join("::");
        Self {
            content,
            key,
            ts: Default::default(),
            source: None,
            process: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // use super::json::*;

    #[test]
    fn test_key() {
        #[derive(Debug, Deserialize, PartialEq, Serialize)]
        struct StructInDataGroup {
            struct_field1: bool,
            struct_field2: f64,
        }

        #[derive(Debug, Deserialize, PartialEq, Serialize)]
        enum DataGroup1 {
            DataGroupF64(f64),
            DataGroupStruct(StructInDataGroup),
        }

        #[derive(Debug, Deserialize, PartialEq, Serialize)]
        enum Data {
            DataUnit(()),
            DataF64(f64),
            DataGroup(DataGroup1),
        }

        let msg = Message::new(Data::DataUnit(()));
        assert_eq!("Data::DataUnit", msg.key);

        let msg = Message::new(Data::DataF64(0.0));
        assert_eq!("Data::DataF64", msg.key);

        let msg = Message::new(Data::DataGroup(DataGroup1::DataGroupF64(0.0)));
        assert_eq!("Data::DataGroup::DataGroupF64", msg.key);

        let msg = Message::new(Data::DataGroup(DataGroup1::DataGroupStruct(
            StructInDataGroup {
                struct_field1: false,
                struct_field2: 0.0,
            },
        )));

        assert_eq!("Data::DataGroup::DataGroupStruct", msg.key);

        let text = msg.serialize().unwrap();
        let msg1 = Message::<Data>::deserialize(&text).unwrap();
        assert_eq!(msg, msg1);
    }
}
