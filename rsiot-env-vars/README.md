Загрузка переменных среды из ОС и из файла .env

Ссылки:

- [Документация docs.rs](https://docs.rs/rsiot-env-vars/latest/)

- [Репозиторий GitHub](https://github.com/Konstantin-Dudersky/rsiot/tree/main/rsiot-env-vars)

- [Примеры](https://github.com/Konstantin-Dudersky/rsiot/tree/main/rsiot-env-vars/examples)

## Команды

### Cоздать / обновить файл .env.example

```bash
cargo run --bin env_vars create
```

Создает файл `.env.example` со значениями по-умолчанию

### Проверить файл .env

```bash
cargo run --bin env_vars check
```

1. Пытается загрузить файл `.env`
2. Читает настройки в структуру `src/config.rs`
