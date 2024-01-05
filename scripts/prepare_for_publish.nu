# Подготовка к публикации

cargo update
cargo outdated
cargo +nightly udeps
cargo check --all-targets
cargo clippy --all-targets
cargo build

do {
    print $"\n\n(ansi magenta_bold) rsiot (ansi reset)\n\n"
    cd rsiot
    cargo rdme --force
}

do {
    print $"\n\n(ansi magenta_bold) rsiot-components-config (ansi reset)\n\n"
    cd rsiot-components-config
    cargo rdme --force
}

do {
    print $"\n\n(ansi magenta_bold) rsiot-esp (ansi reset)\n\n"
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
    print $"\n\n(ansi magenta_bold) rsiot-logging (ansi reset)\n\n"
    cd rsiot-logging
    cargo clippy --target="wasm32-unknown-unknown" -p rsiot-logging
    cargo rdme --force
}

do {
    print $"\n\n(ansi magenta_bold) rsiot-modbus-client (ansi reset)\n\n"
    cd rsiot-modbus-client
    cargo rdme --force
}

do {
    print $"\n\n(ansi magenta_bold) rsiot-plc (ansi reset)\n\n"
    cd rsiot-plc
    cargo rdme --force
}

do {
    print $"\n\n(ansi magenta_bold) rsiot-websocket-server (ansi reset)\n\n"
    cd rsiot-websocket-server
    cargo rdme --force
}

cp rsiot/README.md .
