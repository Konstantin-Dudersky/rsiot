# Подготовка к публикации

cargo update
cargo outdated
cargo +nightly udeps
cargo check --all-targets
cargo clippy --all-targets
cargo build

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
    cargo update
    cargo outdated
    cargo +nightly udeps
    cargo check --all-targets
    cargo clippy --all-targets
    cargo build
    cargo rdme --force
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
