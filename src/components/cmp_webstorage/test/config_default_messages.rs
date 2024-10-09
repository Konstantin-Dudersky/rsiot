use crate::{
    components::cmp_webstorage,
    message::{example_message::*, Message},
};

#[test]
fn default_messages() {
    // Пустой массив
    let default_messages_0 = vec![];
    // Есть значения
    let default_messages_1 = vec![Message::new_custom(Custom::ValueInstantF64(1.2))];

    for default_messages in [default_messages_0, default_messages_1] {
        let _ = cmp_webstorage::Config::<Custom> {
            default_messages,
            ..Default::default()
        };
    }
}
