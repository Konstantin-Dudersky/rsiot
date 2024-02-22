pub fn create_signal_from_msg(input: &str) -> String {
    let parts = &input.split("-").map(String::from).collect::<Vec<String>>();
    let code = r#"
    create_signal_from_msg::create(create_signal_from_msg::Config {
        default: &default,
        fn_input: |msg| {
            let value = &msg.data;
            &fn_input
        },
        fn_output: |value| {
            Some(&fn_output)
        },
    })"#;
    let code = code.replace("&default", &msg_new_full(&parts, "Default::default()"));
    let code = code.replace("&fn_input", &fn_input(&parts));
    let code = code.replace("&fn_output", &msg_new_full(&parts, "value"));
    let code = code.replace('\"', "");
    code.into()
}

fn msg_new_full(parts: &[String], value: &str) -> String {
    let mut token = String::from(value);
    for part in parts.iter().rev() {
        token = format!("{part}({token})");
    }
    token = format!("Message::new_full({token})");
    token
}

fn fn_input(parts: &[String]) -> String {
    let mut token = "Some(value.clone())".into();
    for part in parts.iter().rev() {
        token = format!("match value {{ {}(value) => {}, _ => None, }}", part, token);
    }
    token
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
    default: Message::new_full(MsgData::Custom(ExampleMessage::ValueInstantF64(Default::default()))),
    fn_input: |msg| {
        let value = &msg.data;
        match value { MsgData::Custom(value) => match value { ExampleMessage::ValueInstantF64(value) => Some(value.clone()), _ => None, }, _ => None, }
    },
    fn_output: |value| {
        Some(Message::new_full(MsgData::Custom(ExampleMessage::ValueInstantF64(value))))
    },
})"#;
        let test_out = create_signal_from_msg("MsgData::Custom-ExampleMessage::ValueInstantF64");
        assert_eq!(valid_out, test_out);
    }
}
