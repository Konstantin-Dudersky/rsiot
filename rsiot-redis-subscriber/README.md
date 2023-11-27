Компонент подписки на сообщения из Redis. Используется крейт [redis](https://crates.io/crates/redis).

Пример конфигурации образа docker:

```yaml
redis:
  container_name: redis
  hostname: redis
  image: redis/redis-stack:latest
  ports:
    - "6379:6379" # порт Redis
    - "8001:8001" # порт UI
  volumes:
    - redis_data:/data
    - ./services/redis/redis.conf:/redis-stack.conf
```

## Тестирование

Для тестирования запустить Docker-образ. См. README в корне
