use cfg_aliases::cfg_aliases;

fn main() {
    embuild::espidf::sysenv::output();

    cfg_aliases! {
        aarch64_linux_android: { all (
            target_arch = "aarch64",
            target_vendor = "unknown",
            target_os = "android"
        ) },
        aarch64__unknown__linux__gnu: { all (
            target_arch = "aarch64",
            target_vendor = "unknown",
            target_os = "linux",
            target_env = "gnu"
        ) },
        riscv32imc__esp__espidf: { all (
            target_arch = "riscv32",
            target_vendor = "espressif",
            target_os = "espidf"
        ) },
        wasm32__unknown__unknown: { all (
            target_arch = "wasm32",
            target_vendor = "unknown",
            target_os = "unknown"
        ) },
        x86_64__unknown__linux__gnu: { all(
            target_arch = "x86_64",
            target_vendor = "unknown",
            target_os = "linux",
            target_env = "gnu"
        ) },
    }
}
