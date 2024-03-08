## Установка таргетов

```bash
rustup target add x86_64-unknown-linux-gnu;
rustup target add x86_64-unknown-linux-gnu --toolchain nightly;
rustup target add aarch64-unknown-linux-gnu;
rustup target add aarch64-unknown-linux-gnu --toolchain nightly;
rustup target add wasm32-unknown-unknown;
rustup target add wasm32-unknown-unknown --toolchain nightly;
```

## Разработка

Для запуска необходимых docker-образов:

```bash
docker compose up -d
```

## Просмотр сгенерированной документации

```bash
cargo doc --open
```

## Отладка разных фич

Активировать фичи для помощи rust-analyzer - в корне в папке .vscode:

```json
{
  //   "rust-analyzer.cargo.features": []
  "rust-analyzer.cargo.features": ["single-thread"]
}
```

## Публикация версии на crates.io

Проверяем, что все компилируется без ошибок:

```bash
nu scripts/prepare_for_publish.nu
```

Коммитим все изменения в git.

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
