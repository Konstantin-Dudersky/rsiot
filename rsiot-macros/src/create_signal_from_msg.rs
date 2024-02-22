pub fn create_signal_from_msg(input: &str) -> String {
    let enums = &input.split('-').map(String::from).collect::<Vec<String>>();
    let enums = add_enum_names(&enums);
    let code = r#"
create_signal_from_msg::create(create_signal_from_msg::Config {
    default: Message::new_full(&default),
    fn_input: |msg| {
        let value = &msg.data;
        match value {
            &fn_input => Some(value.clone()),
            _ => None,
        }
    },
    fn_output: |value| {
        Some(Message::new_full(&fn_output))
    },
})"#;
    let code = code.replace("&default", &msg_new_full(&enums, "Default::default()"));
    let code = code.replace("&fn_input", &msg_new_full(&enums, "value"));
    let code = code.replace("&fn_output", &msg_new_full(&enums, "value"));
    code.replace('\"', "")
}

fn msg_new_full(parts: &[String], value: &str) -> String {
    let mut token = String::from(value);
    for part in parts.iter().rev() {
        token = format!("{part}({token})");
    }
    token
}

/// Добавляет названия перечислений к вариантам
///
/// Т.е. из строки вида `Custom-ValueInstantF64`
/// получается строка вида `MsgType::Custom-Custom::ValueInstantF64`
fn add_enum_names(enum_variants: &[String]) -> Vec<String> {
    let mut new_enum_variants = vec![];
    let mut enum_name = String::from("MsgType");
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
    fn test1() {
        let valid_out = r#"
create_signal_from_msg::create(create_signal_from_msg::Config {
    default: Message::new_full(MsgType::Custom(Custom::ValueInstantF64(Default::default()))),
    fn_input: |msg| {
        let value = &msg.data;
        match value {
            MsgType::Custom(Custom::ValueInstantF64(value)) => Some(value.clone()),
            _ => None,
        }
    },
    fn_output: |value| {
        Some(Message::new_full(MsgType::Custom(Custom::ValueInstantF64(value))))
    },
})"#;
        let test_out = create_signal_from_msg("Custom-ValueInstantF64");
        assert_eq!(valid_out, test_out);
    }
}
