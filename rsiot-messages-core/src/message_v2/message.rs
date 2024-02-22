use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::{super::msg_meta::Timestamp, MsgDataBound, MsgSource};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SystemData {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum MsgData<TCustom> {
    System(SystemData),
    Custom(TCustom),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Message<TData> {
    pub data: MsgData<TData>,
    pub key: String,
    pub ts: Timestamp,
    pub source: Option<MsgSource>,
    pub process: Option<MsgSource>,
}

impl<TCustom> Message<TCustom>
where
    TCustom: MsgDataBound,
{
    pub fn new(custom_data: TCustom) -> Self {
        let data = MsgData::Custom(custom_data);
        let key = define_key(&data);
        Self {
            data,
            key,
            ts: Default::default(),
            source: None,
            process: None,
        }
    }

    pub fn new_custom(custom_data: TCustom) -> Self {
        Self::new(custom_data)
    }

    pub fn new_full(data: MsgData<TCustom>) -> Self {
        let key = define_key(&data);
        Self {
            data,
            key,
            ts: Default::default(),
            source: None,
            process: None,
        }
    }

    pub fn cmp_set(&mut self, cmp: &MsgSource) {
        if self.source.is_none() {
            self.source = Some(cmp.clone());
        }
        self.process = Some(cmp.clone());
    }

    pub fn get_data(&self) -> Option<TCustom> {
        match &self.data {
            MsgData::System(_) => None,
            MsgData::Custom(data) => Some(data.clone()),
        }
    }
}

fn define_key<TCustom>(data: &MsgData<TCustom>) -> String
where
    TCustom: MsgDataBound,
{
    let full_str = format!("{:?}", data);
    let key = full_str.split("(").into_iter().collect::<Vec<&str>>();
    // Убираем последний элемент. Если тип unit (), нужно убрать два последних элемента
    let skip = if key[key.len() - 2] == "" { 2 } else { 1 };
    let key = key[0..key.len() - skip].join("-");
    key
}