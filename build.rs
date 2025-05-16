use cfg_aliases::cfg_aliases;

fn main() {
    // riscv32imc-esp-espidf
    embuild::espidf::sysenv::output();

    cfg_aliases! {
        aarch64_linux_android: { all (
            target_arch = "aarch64",
            target_vendor = "unknown",
            target_os = "android"
        ) },
        aarch64_unknown_linux_gnu: { all (
            target_arch = "aarch64",
            target_vendor = "unknown",
            target_os = "linux",
            target_env = "gnu"
        ) },
        armv7_unknown_linux_gnueabihf: { all (
            target_arch = "arm",
            target_vendor = "unknown",
            target_os = "linux"
        ) },
        riscv32imc_esp_espidf: { all (
            target_arch = "riscv32",
            target_vendor = "espressif",
            target_os = "espidf"
        ) },
        wasm32_unknown_unknown: { all (
            target_arch = "wasm32",
            target_vendor = "unknown",
            target_os = "unknown"
        ) },
        x8664_unknown_linux_gnu: { all(
            target_arch = "x86_64",
            target_vendor = "unknown",
            target_os = "linux",
            target_env = "gnu"
        ) },
    }
}
