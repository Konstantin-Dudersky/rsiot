Компонент сохранения данных в БД TimescaleDB.

Для сохранения используется библиотека [sqlx](https://crates.io/crates/sqlx)

## Разработка

Запустить тестовую базу данных из файла docker compose - см README в корне

Задать переменную окружения в файле .env:

```env
DATABASE_URL = 'postgres://postgres:postgres@localhost:5432/db_data_test'
```
