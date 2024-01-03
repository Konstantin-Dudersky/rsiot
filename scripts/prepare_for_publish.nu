# Подготовка к публикации

cargo update
cargo build
cargo check --all-targets
cargo clippy --all-targets

do {
    cd rsiot
    cargo rdme --force
}

do {
    cd rsiot-components-config
    cargo rdme --force
}

do {
    cd rsiot-esp
    cargo rdme --force
    cargo update
    cargo build
    cargo check --all-targets
    cargo clippy --all-targets
    cargo +nightly udeps
}

do {
    cd rsiot-modbus-client
    cargo rdme --force
}

do {
    cd rsiot-plc
    cargo rdme --force
}

do {
    cd rsiot-websocket-server
    cargo rdme --force
}

cp rsiot/README.md .

# проверяем ненужные зависимости
cargo +nightly udeps
