## Разработка

Для запуска необходимых docker-образов:

```bash
docker compose up -d
```

## Публикация версии на crates.io

Проверяем, что все компилируется без ошибок:

```bash
nu scripts/prepare_for_publish.nu
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

После этого публикуем связанные проекты:

- [rsiot-esp](../rsiot-esp/README.md)
