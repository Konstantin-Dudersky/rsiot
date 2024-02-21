pub fn create_signal_from_msg(input: &str) -> String {
    let parts = &input
        .split("-")
        .map(String::from)
        .collect::<Vec<String>>();

    let tmpl = r#"
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
    let tmpl = tmpl.replace("&default", &msg_create(&parts, "Default::default()"));
    let tmpl = tmpl.replace("&fn_output", &msg_create(&parts, "value"));
    tmpl.into()
}

fn msg_create(parts: &[String], value: &str) -> String {
    format!(
        "Message::new_full({}({})){}",
        parts.join("("),
        value,
        ")".repeat(parts.len() - 1)
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let valid_out = r#"
create_signal_from_msg::create(create_signal_from_msg::Config {
    default: Message::new_full(MsgData::Custom(ExampleMessage::ValueInstantF64(Default::default()))),
    fn_input: |msg| {
        let value = &msg.data;
        match value {
            MsgData::Custom(value) => match value {
                ExampleMessage::ValueInstantF64(value) => Some(value.clone()),
                _ => None,
            },
            _ => None,
        }
    },
    fn_output: |value| {
        Some(Message::new_full(MsgData::Custom(ExampleMessage::ValueInstantF64(value))))
    },
})"#;
        let test_out = create_signal_from_msg("MsgData::Custom-ExampleMessage::ValueInstantF64");
        assert_eq!(valid_out, test_out);
    }
}
