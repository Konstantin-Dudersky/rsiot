Построение интерфейса с помощью leptos.

TODO - скрипт для копирования начальных файлов из git (index.html, etc)

Установить npm-пакеты

```bash
npm install
```

Сборка проекта в релиз

```bash
do {
    cd webapp_folder
    npx tailwindcss -o ./style/output.css --minify
    trunk build --release
}
```
