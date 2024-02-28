pub fn create_signal_from_msg(input: &str) -> String {
    let enum_variants = parse_enum_variants(input);
    let enum_with_names = add_enum_names(&enum_variants);
    let code = r#"
create_signal_from_msg::create(create_signal_from_msg::Config {
    default: &default,
    fn_input: |msg| {
        let value = &msg.data;
        match value {
            &fn_input => Some(value.clone()),
            _ => None,
        }
    },
    fn_output: |value| {
        Some(&fn_output)
    },
})"#;
    let code = code.replace(
        "&default",
        &message_new_from_enums(&enum_with_names, "Default::default()"),
    );
    let code = code.replace("&fn_input", &create_msg_data(&enum_with_names, "value"));
    let code = code.replace(
        "&fn_output",
        &message_new_from_enums(&enum_with_names, "value"),
    );
    code.replace('\"', "")
}

/// Создание сообщения из строки вида `Variant1-Variant2-Variant3::value`
/// TODO - возможно, убрать value и подразумевать по-умолчанию?
pub fn message_new(input: &str) -> String {
    let enum_variants_and_value = input.split("::").collect::<Vec<&str>>();

    let enum_variants = enum_variants_and_value[0];
    let enum_variants = parse_enum_variants(enum_variants);
    let enum_with_names = add_enum_names(&enum_variants);

    let value = enum_variants_and_value[1];

    let code = message_new_from_enums(&enum_with_names, value);
    code.replace('\"', "")
}

fn parse_enum_variants(input: &str) -> Vec<String> {
    input.split('-').map(String::from).collect::<Vec<String>>()
}

/// Создание данных сообщения
fn create_msg_data(parts: &[String], value: &str) -> String {
    let mut token = String::from(value);
    for part in parts.iter().rev() {
        token = format!("{part}({token})");
    }
    token
}

/// Создание сообщения
fn message_new_from_enums(enums: &[String], value: &str) -> String {
    let msg_data = create_msg_data(enums, value);
    format!("Message::new({msg_data})")
}

/// Добавляет названия перечислений к вариантам
///
/// Т.е. из строки вида `Custom-ValueInstantF64`
/// получается строка вида `MsgData::Custom-Custom::ValueInstantF64`
fn add_enum_names(enum_variants: &[String]) -> Vec<String> {
    let mut new_enum_variants = vec![];
    let mut enum_name = String::from("MsgData");
    for enum_variant in enum_variants {
        let new_add = enum_variant.clone();
        new_enum_variants.push(format!("{enum_name}::{enum_variant}"));
        enum_name = new_add;
    }
    new_enum_variants
}

#[cfg(test)]
mod tests {
    //! Запуск:
    //!
    //! ```bash
    //!
    //! ```

    use super::*;

    #[test]
    fn test_create_signal_from_msg() {
        let valid_out = r#"
create_signal_from_msg::create(create_signal_from_msg::Config {
    default: Message::new(MsgData::Custom(Custom::ValueInstantF64(Default::default()))),
    fn_input: |msg| {
        let value = &msg.data;
        match value {
            MsgData::Custom(Custom::ValueInstantF64(value)) => Some(value.clone()),
            _ => None,
        }
    },
    fn_output: |value| {
        Some(Message::new(MsgData::Custom(Custom::ValueInstantF64(value))))
    },
})"#;
        let test_out = create_signal_from_msg("Custom-ValueInstantF64");
        assert_eq!(valid_out, test_out);
    }

    #[test]
    fn test_message_new() {
        let valid_out = "Message::new(MsgData::Custom(Custom::ValueInstantF64(123.0)))";
        let test_out = message_new("Custom-ValueInstantF64::123.0");
        assert_eq!(valid_out, test_out);
    }
}
