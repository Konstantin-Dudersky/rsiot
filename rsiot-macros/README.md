Проверка сгенерированного кода - [cargo-expand](https://github.com/dtolnay/cargo-expand)

```bash
cargo expand message::example_message | save -f expand.rs

cargo expand -p rsiot-messages-core example_message

cargo expand -p rsiot-leptos --example create_signal_from_msg --target wasm32-unknown-unknown

cargo expand -p rsiot-messages-core --example message_new_macro
```

https://stackoverflow.com/questions/65182338/how-to-create-a-macro-that-matches-enum-variants-without-knowing-its-structure

Макрос для генерирования метода `into_eav` для типа сообщения. Сейчас не исользуется - слишком много
различий между сообщениями
