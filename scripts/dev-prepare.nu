rustup target add aarch64-linux-android
rustup target add aarch64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
rustup target add wasm32-unknown-unknown
rustup target add x86_64-unknown-linux-gnu

# Требуется для сборки cmp_mqtt_client для ARMv7
cargo install --force --locked bindgen-cli
