## Установка таргетов

```bash
rustup target add x86_64-unknown-linux-gnu;
rustup target add x86_64-unknown-linux-gnu --toolchain nightly;
rustup target add aarch64-unknown-linux-gnu;
rustup target add aarch64-unknown-linux-gnu --toolchain nightly;
rustup target add wasm32-unknown-unknown;
rustup target add wasm32-unknown-unknown --toolchain nightly;
```

sudo apt install clang

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

- Проверяем, что все компилируется без ошибок:

  ```bash
  nu scripts/prepare_for_publish.nu
  ```

- Коммитим все изменения в git.

  - rsiot-macros
  - rsiot

- Изменяем номер версии проекта в Cargo.toml.

- Публикуем на `crates.io`:

  ```bash
  cargo publish --allow-dirty
  ```
