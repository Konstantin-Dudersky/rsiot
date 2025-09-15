use std::collections::HashMap;

use crate::message::MsgDataBound;

/// Конфигурация компонента cmp_svg
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    pub file: &'static str,

    pub fn_input: fn(&TMsg) -> Vec<SvgChange>,

    pub fn_output: fn(&[u8]) -> TMsg,
}

// Изменение атрибута
// Изменение атрибута (style)
// Изменение текста
// Изменение атрибута вложенного элемента
// Изменение атрибута (style) вложенного элемента
// Изменение текста вложенного элемента

#[derive(Clone, Debug)]
pub struct SvgChange {
    pub id: &'static str,
    pub change: Vec<SvgChangeType>,
    pub change_childs: Option<HashMap<String, Vec<SvgChangeType>>>,
}

#[derive(Clone, Debug)]
pub enum SvgChangeType {
    ChangeAttr {
        attr_name: &'static str,
        new_value: String,
    },
    ChangeAttrStyle,
    ChangeText {
        text: String,
    },
}
