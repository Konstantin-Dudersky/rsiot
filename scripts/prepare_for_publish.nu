# Подготовка к публикации

def print_header [header: string] {
    print $"\n\n(ansi magenta_bold)($header)(ansi reset)\n\n"
}

print_header "workspace - update"
cargo update

print_header "workspace - outdated"
cargo outdated

print_header "workspace - unused dependencies"
# cargo +nightly udeps
# TODO - разобраться с ошибками

print_header "workspace clippy - x86_64-unknown-linux-gnu / multi-thread"
cargo clippy --all-targets --target="x86_64-unknown-linux-gnu" --features=""

print_header "workspace clippy - x86_64-unknown-linux-gnu / single-thread"
cargo clippy --all-targets --target="x86_64-unknown-linux-gnu" --features="single-thread"

# print_header "workspace clippy - aarch64-unknown-linux-gnu / multi-thread"
# cargo clippy --all-targets --target="aarch64-unknown-linux-gnu" --features=""
# TODO - настроить проверки по остальным таргетам

print_header "workspace clippy - wasm32-unknown-unknown / single-thread"
cargo clippy --all-targets --target="wasm32-unknown-unknown" --features="single-thread"

# print_header "workspace clippy - wasm32-unknown-unknown / single-thread"
# cargo clippy --all-targets --target="wasm32-unknown-unknown" --features=""

do {
    print_header "rsiot"
    cd rsiot
    cargo rdme --force
}

do {
    print_header "rsiot-components-config"
    cd rsiot-components-config
    cargo rdme --force
}

do {
    print_header "rsiot-logging"
    cd rsiot-logging
    cargo clippy --target="wasm32-unknown-unknown" -p rsiot-logging
    cargo rdme --force
}

do {
    print_header "rsiot-http-client-wasm"
    cd rsiot-logging
    cargo clippy -p rsiot-http-client-wasm --features single-thread --target wasm32-unknown-unknown
    cargo rdme --force
}

do {
    print_header "rsiot-modbus-client"
    cd rsiot-modbus-client
    cargo rdme --force
}

do {
    print_header "rsiot-http-server"
    cd rsiot-http-server
    cargo rdme --force
}

do {
    print_header "rsiot-plc"
    cd rsiot-plc
    cargo rdme --force
}

do {
    print_header "rsiot-websocket-server"
    cd rsiot-websocket-server
    cargo rdme --force
}

do {
    print_header "rsiot-esp"
    cd rsiot-esp
    cargo update
    cargo outdated
    cargo +nightly udeps
    cargo check --all-targets
    cargo clippy --all-targets
    cargo build
    cargo rdme --force
}

cp rsiot/README.md .
