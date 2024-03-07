# Подготовка к публикации

def print_header [header: string] {
    print $"\n\n(ansi magenta_bold)($header)(ansi reset)\n\n"
}

# cargo clippy -------------------------------------------------------------------------------------

print_header "workspace clippy - x86_64-unknown-linux-gnu / multi-thread"
cargo clippy --all-targets --target="x86_64-unknown-linux-gnu" --features="cmp_auth"

print_header "workspace clippy - x86_64-unknown-linux-gnu / single-thread"
cargo clippy --all-targets --target="x86_64-unknown-linux-gnu" --features="cmp_auth, single-thread"

print_header "workspace clippy - aarch64-unknown-linux-gnu / multi-thread"
cargo clippy --all-targets --target="aarch64-unknown-linux-gnu" --features="cmp_auth"

print_header "workspace clippy - aarch64-unknown-linux-gnu / single-thread"
cargo clippy --all-targets --target="aarch64-unknown-linux-gnu" --features="cmp_auth, single-thread"

# print_header "workspace clippy - wasm32-unknown-unknown / multi-thread"
# cargo clippy --all-targets --target="wasm32-unknown-unknown" --features=""
# TODO - настроить проверки по остальным таргетам

print_header "workspace clippy - wasm32-unknown-unknown / single-thread"
cargo clippy --all-targets --target="wasm32-unknown-unknown" --features="single-thread"


# cargo udeps --------------------------------------------------------------------------------------

# print_header "workspace udeps - x86_64-unknown-linux-gnu / multi-thread"
# cargo +nightly udeps --target="x86_64-unknown-linux-gnu" --features="cmp_auth"

# print_header "workspace udeps - x86_64-unknown-linux-gnu / single-thread"
# cargo +nightly udeps --target="x86_64-unknown-linux-gnu" --features="cmp_auth, single-thread"

# print_header "workspace udeps - aarch64-unknown-linux-gnu / multi-thread"
# cargo +nightly udeps --target="aarch64-unknown-linux-gnu" --features=""

# print_header "workspace udeps - aarch64-unknown-linux-gnu / single-thread"
# cargo +nightly udeps --target="aarch64-unknown-linux-gnu" --features="single-thread"

# print_header "workspace udeps - wasm32-unknown-unknown / multi-thread"
# cargo +nightly udeps --target="wasm32-unknown-unknown" --features=""

# print_header "workspace udeps - wasm32-unknown-unknown / single-thread"
# cargo +nightly udeps --target="wasm32-unknown-unknown" --features="single-thread"


# cargo update -------------------------------------------------------------------------------------

print_header "workspace - update"
cargo update


# cargo outdated -----------------------------------------------------------------------------------

print_header "workspace - outdated"
cargo outdated


# cargo test ---------------------------------------------------------------------------------------

# print_header "workspace test - x86_64-unknown-linux-gnu / multi-thread"
# cargo test --all-targets --target="x86_64-unknown-linux-gnu" --features=""
# TODO - тесты не проходят

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
    cargo rdme --force
}

do {
    print_header "rsiot-http-client-wasm"
    cd rsiot-http-client-wasm
    cargo rdme --force
}

do {
    print_header "rsiot-messages-core"
    cd rsiot-messages-core
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
