# Подготовка к публикации

cargo clippy
cargo build

do {
    cd rsiot
    cargo rdme --force
}

do {
    cd rsiot-gpio-read-esp
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
