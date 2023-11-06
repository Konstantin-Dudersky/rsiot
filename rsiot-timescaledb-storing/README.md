Компонент сохранения данных в БД TimescaleDB.

Для сохранения используется библиотека [sqlx](https://crates.io/crates/sqlx)

## Разработка

Запустить тестовую базу данных:

```bash
docker compose -f rsiot-timescaledb-storing/docker-compose.yml up -d
```

Задать переменную окружения:

```nu
$env.DATABASE_URL = 'postgres://postgres:postgres@localhost:5432/db_data_test'
```
