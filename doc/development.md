## Разработка

Для запуска необходимых docker-образов:

```bash
docker compose --profile dev up -d
```

## Публикация версии на crates.io

Проверяем, что все компилируется без ошибок:

```bash
cargo build
```

Изменяем номер версии проекта:

```bash
cargo ws version patch --no-git-push
# или major, minor, patch
```

Публикуем на `crates.io`:

```bash
cargo ws publish --from-git --allow-dirty
```

## Идеи для улучшения

TODO - заменить указатели функций на трейты Fn

TODO - обновить до hyper версии 1.0 - какие-то ошибки. Подождать обновления axum до 0.7
