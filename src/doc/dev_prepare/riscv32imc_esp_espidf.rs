//! # riscv32imc-esp-espidf
//!
//! ## Подготовка среды разработки
//!
//! **toolchain**
//!
//! ```bash
//! rustup toolchain install nightly-unknown-linux-gnu --component rust-src
//! ```
//!
//! **ESP-IDF**
//!
//! Зависимости для ESP-IDF <https://docs.espressif.com/projects/esp-idf/en/latest/esp32/get-started/linux-macos-setup.html#for-linux-users>:
//!
//! ```bash
//! sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
//! ```
//!
//! **ldproxy**
//!
//! ```bash
//! cargo install ldproxy
//! ```
//!
//! **LLVM**
//!
//! ```bash
//! sudo bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
//! ```
//!
//! **espflash**
//!
//! ```bash
//! sudo usermod -a -G dialout $USER
//!
//! sudo apt install libudev-dev
//!
//! cargo install espflash
//! ```
//!
//! **cargo generate**
//!
//! ```bash
//! cargo install cargo-generate
//! ```
//!
//! ## Создание проекта
//!
//! ```bash
//! cargo generate esp-rs/esp-idf-template cargo
//! ```
//!
//! **`.zed/settings.json`**
//!
//! ```json
//! {
//!   "lsp": {
//!     "rust-analyzer": {
//!       "initialization_options": {
//!         "check": {
//!           "command": "clippy"
//!         },
//!         "cargo": {
//!           "target": "riscv32imc-esp-espidf"
//!         }
//!       }
//!     }
//!   }
//! }
//! ```
//!
//! **`.cargo/config.toml`**
//!
//! ```toml
//! [build]
//! target = "riscv32imc-esp-espidf"
//!
//! [target.riscv32imc-esp-espidf]
//! linker = "ldproxy"
//! runner = "espflash flash --monitor"
//! rustflags = [
//!     "--cfg",
//!     "espidf_time64",
//!     "--cfg",
//!     "mio_unsupported_force_poll_poll", # https://github.com/tokio-rs/tokio/issues/5866
//! ]
//!
//! [unstable]
//! build-std = ["std", "panic_abort"]
//!
//! [env]
//! MCU = "esp32c3"
//! # install all libraries globally, in folder ~/.espressif
//! ESP_IDF_TOOLS_INSTALL_DIR = "global"
//! # check current version on https://github.com/espressif/esp-idf/releases
//! ESP_IDF_VERSION = "v5.3.1"
//! ```
//!
//! **`build.rs`**
//!
//! ```ignore
//! fn main() {
//!     embuild::espidf::sysenv::output();
//! }
//! ```
//!
//! **`rust-toolchain.toml`**
//!
//! ```toml
#![doc = include_str!("../../../rust-toolchain.toml")]
//! ```
//!
//! **`sdkconfig.defaults`**
//!
//! ```toml
#![doc = include_str!("../../../sdkconfig.defaults")]
//! ```
//!
//! **`Cargo.toml`**
//!
//! ```toml
//! [dependencies]
//! esp-idf-svc = { version = "*" }
//!
//! [build-dependencies]
//! embuild = { version = "*", features = ["espidf"] }
//! ```
//!
//! **`main.rs`**
//!
//! Первой строкой в функции main():
//!
//! ```rust
//! esp_idf_svc::sys::link_patches();
//! ```
