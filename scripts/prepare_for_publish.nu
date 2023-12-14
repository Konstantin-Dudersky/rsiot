# Подготовка к публикации

cargo clippy
cargo build

do {
    cd rsiot
    cargo rdme --force
}

do {
    cd rsiot-modbus-client
    cargo rdme --force
}

do {
    cd rsiot-websocket-server
    cargo rdme --force
}

cp rsiot/README.md .