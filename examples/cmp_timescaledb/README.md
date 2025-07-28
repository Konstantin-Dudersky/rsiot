Запустить базу данных и pgadmin:

```sh
cd examples/cmp_timescaledb
docker-compose up
```

Запустить пример:

```sh
RUST_LOG=debug cargo run --example cmp_timescaledb --features "cmp_timescaledb"
```

SQL-скрипт для чтения данных из TimescaleDB:

```sql
SELECT "time", value
FROM raw
ORDER BY "time" DESC
LIMIT 5000
```
