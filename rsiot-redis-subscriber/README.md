Компонент подписки на сообщения из Redis. Используется крейт [redis](https://crates.io/crates/redis).

## Тестирование

```bash
docker run -d --name redis-stack -p 6379:6379 -p 4000:4000 redis/redis-stack:latest
```