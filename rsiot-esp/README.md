<!-- cargo-rdme start -->

Чтение входов и запись выходов GPIO микроконтроллера ESP.

Тестируется с ESP32-C3 и ESP32-S3.

TODO - В данный момент значение с пинов считывается циклически. Возможно, стоит переделать на
считывание по подписке.

<!-- cargo-rdme end -->

## Публикация версии на crates.io

Обновляем версии зависимостей `Cargo.toml` пакетов `rsiot-*`.

Увеличиваем номер версии пакета в `Cargo.toml`. Задать равный текущей версии `rsiot`.

Публикуем версию:

```bash
cargo publish --allow-dirty --no-verify
```

## Сделать

- TODO http-client-esp
- TODO ws-client
- TODO mqtt 1
- TODO mqtt 2
