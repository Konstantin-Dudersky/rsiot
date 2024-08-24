use crate::message::*;

// ANCHOR: TFnInput
type TFnInput<TMsg> = fn(Message<TMsg>) -> Result<Option<ConfigWebstorageItem>, anyhow::Error>;
// ANCHOR_END: TFnInput

// ANCHOR: TFnOutput
type TFnOutput<TMsg> = fn(ConfigWebstorageItem) -> Result<Option<Message<TMsg>>, anyhow::Error>;
// ANCHOR_END: TFnOutput

/// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#Config
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#kind
    pub kind: ConfigKind,

    /// Сохранение сообщений в хранилище
    pub fn_input: TFnInput<TMsg>,

    /// Загрузка сообщений из хранилища
    pub fn_output: TFnOutput<TMsg>,

    /// Значения по-умолчанию, когда хранилище пустое
    pub default_items: Vec<ConfigWebstorageItem>,
}

/// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#ConfigKind
pub enum ConfigKind {
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#LocalStorage
    LocalStorage,
    /// https://konstantin-dudersky.github.io/rsiot-docs/1_components/cmp_webstorage.html#SessionStorage
    SessionStorage,
}

/// Структура для представления данных, хранящихся в WebStorage
pub struct ConfigWebstorageItem {
    /// Ключ
    pub key: String,
    /// Значение
    pub value: String,
}

#[cfg(test)]
mod tests {
    use crate::{components::cmp_webstorage, message::example_message::*};

    use super::{ConfigWebstorageItem, Message};

    #[test]
    #[allow(clippy::no_effect)]
    fn fn_all() {
        cmp_webstorage::Config::<Custom> {
            // ANCHOR: kind
            kind: cmp_webstorage::ConfigKind::SessionStorage,
            // ANCHOR_END: kind

            // ANCHOR: fn_input_save_all
            fn_input: |msg| {
                let key = msg.key.clone();
                let value = msg.serialize()?;
                let item = ConfigWebstorageItem { key, value };
                Ok(Some(item))
            },
            // ANCHOR_END: fn_input_save_all

            // ANCHOR: fn_output_not_load
            fn_output: |_| Ok(None),
            // ANCHOR_END: fn_output_not_load

            // ANCHOR: default_items_empty
            default_items: vec![],
            // ANCHOR_END: default_items_empty
        };
    }

    #[test]
    #[allow(clippy::no_effect)]
    fn fn_filter() {
        cmp_webstorage::Config::<Custom> {
            kind: cmp_webstorage::ConfigKind::SessionStorage,

            // ANCHOR: fn_input_filter
            fn_input: |msg| {
                let Some(msg_custom) = msg.get_custom_data() else {
                    return Ok(None);
                };
                let item = match msg_custom {
                    Custom::ValueInstantString(value) => cmp_webstorage::ConfigWebstorageItem {
                        key: "save_item".into(),
                        value: value.to_string(),
                    },
                    _ => return Ok(None),
                };
                Ok(Some(item))
            },
            // ANCHOR_END: fn_input_filter

            // ANCHOR: fn_output_filter
            fn_output: |item| {
                let data = match item.key.as_str() {
                    "save_item" => Custom::ValueInstantString(item.value),
                    _ => return Ok(None),
                };
                let msg = Message::new_custom(data);
                Ok(Some(msg))
            },
            // ANCHOR_END: fn_output_filter

            // ANCHOR: default_items_non_empty
            default_items: vec![ConfigWebstorageItem {
                key: "save_item".into(),
                value: "default".into(),
            }],
            // ANCHOR_END: default_items_non_empty
        };
    }
}
