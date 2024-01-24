Проверка сгенерированного кода - [cargo-expand](https://github.com/dtolnay/cargo-expand)

```bash
cargo expand -p rsiot-messages-core example_message
```

https://stackoverflow.com/questions/65182338/how-to-create-a-macro-that-matches-enum-variants-without-knowing-its-structure

Макрос для генерирования метода `into_eav` для типа сообщения. Сейчас не исользуется - слишком много
различий между сообщениями
