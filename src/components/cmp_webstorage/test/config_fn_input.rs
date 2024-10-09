use crate::{
    components::cmp_webstorage,
    message::{example_message::*, Message},
};

#[test]
fn fn_input() {
    // Сохраняем все сообщения
    let fn_input_0 = |msg: Message<Custom>| Some(msg.clone());
    // Не сохранять ничего
    let fn_input_1 = |_| None;

    for fn_input in [fn_input_0, fn_input_1] {
        let _ = cmp_webstorage::Config::<Custom> {
            fn_input,
            ..Default::default()
        };
    }
}
