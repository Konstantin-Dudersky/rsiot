use std::collections::HashMap;

use crate::message::MsgDataBound;

// ANCHOR: Config
/// Конфигурация компонента cmp_svg
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Файл с документом SVG
    ///
    /// Можно прочитать:
    ///
    /// ```rs
    /// include_str!("../files/input.svg")
    /// ```
    pub file: &'static str,

    /// Функция преобразования входящих сообщений в изменения SVG
    pub fn_input: fn(&TMsg) -> Vec<SvgChange>,

    /// Функция преобразования изменённого файла SVG в исходящее сообщение.
    ///
    /// Файл передаётся как `Vec<u8>`
    pub fn_output: fn(&[u8]) -> TMsg,
}
// ANCHOR: Config

// ANCHOR: SvgChange
/// Изменения элемента SVG
#[derive(Clone, Debug)]
pub struct SvgChange {
    /// Идентификатор элемента SVG
    pub id: &'static str,

    /// Вектор изменений
    pub change: Vec<SvgChangeType>,

    /// Изменение вложенных элементов TODO
    pub change_childs: Option<HashMap<String, Vec<SvgChangeType>>>,
}
// ANCHOR: SvgChange

// ANCHOR: SvgChangeType
/// Типы изменений элемента SVG
#[derive(Clone, Debug)]
pub enum SvgChangeType {
    /// Изменение атрибута
    ChangeAttr {
        /// Название атрибута
        attr_name: &'static str,
        /// Новое значение атрибута
        new_value: String,
    },

    /// Изменение одного параметра в атрибуте style
    ChangeAttrStyle {
        /// Параметр стиля
        attr_style_name: &'static str,
        /// Новое значение параметра стиля
        new_value: String,
    },

    /// Изменение текста элемента
    ChangeText {
        /// Новое значение текста
        text: String,
    },
}
// ANCHOR: SvgChangeType
