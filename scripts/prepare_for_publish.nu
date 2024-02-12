# Подготовка к публикации

def print_header [header: string] {
    print $"\n\n(ansi magenta_bold) ($header) (ansi reset)\n\n"
}

print_header "workspace - update"
cargo update

print_header "workspace - outdated"
cargo outdated

print_header "workspace - unused dependencies"
# cargo +nightly udeps

print $"\n\n(ansi magenta_bold) workspace - check (ansi reset)\n\n"
cargo check --all-targets

print $"\n\n(ansi magenta_bold) workspace - clippy \(multi-thread\) (ansi reset)\n\n"
cargo clippy --all-targets --features=""

print $"\n\n(ansi magenta_bold) workspace - clippy \(single-thread\) (ansi reset)\n\n"
cargo clippy --all-targets --features="single-thread"

print $"\n\n(ansi magenta_bold) workspace - build \(multi-thread\) (ansi reset)\n\n"
cargo build --all-targets --features=""

print $"\n\n(ansi magenta_bold) workspace - build \(single-thread\) (ansi reset)\n\n"
cargo build --all-targets --features="single-thread"

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
