# Подготовка к публикации

print $"\n\n(ansi magenta_bold) workspace - update (ansi reset)\n\n"
cargo update

print $"\n\n(ansi magenta_bold) workspace - outdated (ansi reset)\n\n"
cargo outdated

print $"\n\n(ansi magenta_bold) workspace - unused dependencies (ansi reset)\n\n"
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

do {
    # TODO - https://github.com/rust-lang/rust/pull/119632
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

cp rsiot/README.md .
