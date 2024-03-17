# Подготовка к публикации

def print_header [header: string] {
    print $"\n\n(ansi magenta_bold)($header)(ansi reset)\n\n"
}

cd rsiot


let features = [
    {
        name: "cmp_auth",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_http_client",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_http_client_wasm",
        targets: [
            "wasm32-unknown-unknown",
        ],
    },
    {
        name: "cmp_http_server",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_influxdb",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_leptos",
        targets: [
            "wasm32-unknown-unknown",
        ],
    },
    {
        name: "cmp_modbus_client",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_plc",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
            "wasm32-unknown-unknown",
        ],
    },
    {
        name: "cmp_redis_client",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_storage_esp",
        targets: [
            "riscv32imc-esp-espidf",
        ],
    },
    {
        name: "cmp_surrealdb",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_timescaledb",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_websocket_client",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_websocket_client_wasm",
        targets: [
            "wasm32-unknown-unknown",
        ],
    },
    {
        name: "cmp_websocket_server",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "cmp_webstorage",
        targets: [
            "wasm32-unknown-unknown",
        ],
    },
    {
        name: "env_vars",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
        ],
    },
    {
        name: "executor",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
            "wasm32-unknown-unknown",
        ],
    },
    {
        name: "logging",
        targets: [
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-gnu",
            "wasm32-unknown-unknown",
        ],
    },
]

# cargo clippy -------------------------------------------------------------------------------------

for feat in $features {
    for target in $feat.targets {
        let add_feats = match $target {
            "aarch64-unknown-linux-gnu" => ["", "single-thread"], 
            "riscv32imc-esp-espidf" => ["single-thread"],
            "x86_64-unknown-linux-gnu" => ["", "single-thread"],
            "wasm32-unknown-unknown" => ["single-thread"],
        }
        let toolchain = match $target {
            "aarch64-unknown-linux-gnu" => "+stable",
            "riscv32imc-esp-espidf" => "+nightly-2024-02-01-x86_64-unknown-linux-gnu",
            "x86_64-unknown-linux-gnu" => "+stable",
            "wasm32-unknown-unknown" => "+stable",
            _ => "",
        };
        for add_feat in $add_feats {
            print_header $"workspace clippy - ($feat.name) / ($target) / ($add_feat)";
            let command = $'cargo ($toolchain) clippy --all-targets --target="($target)" --features="($feat.name), ($add_feat)"';
            print $"execute command: ($command)";
            nu -c $command;
        }
    }
}

# cargo doc ----------------------------------------------------------------------------------------

rm -rf ../../rsiot-docs/rustdoc; mkdir ../../rsiot-docs/rustdoc/riscv32imc-esp-espidf
cargo doc --target riscv32imc-esp-espidf --features="cmp_esp" --no-deps -Zunstable-options -Zrustdoc-scrape-examples
cp -r target/riscv32imc-esp-espidf/doc/* ../../rsiot-docs/rustdoc/riscv32imc-esp-espidf

# cargo udeps --------------------------------------------------------------------------------------

for feat in $features {
    for target in $feat.targets {
        let add_feats = match $target {
            "aarch64-unknown-linux-gnu" => ["", "single-thread"], 
            "riscv32imc-esp-espidf" => ["single-thread"],
            "x86_64-unknown-linux-gnu" => ["", "single-thread"],
            "wasm32-unknown-unknown" => ["single-thread"],
        }
        let toolchain = match $target {
            "aarch64-unknown-linux-gnu" => "+nightly-2024-02-01-x86_64-unknown-linux-gnu",
            "riscv32imc-esp-espidf" => "+nightly-2024-02-01-x86_64-unknown-linux-gnu",
            "x86_64-unknown-linux-gnu" => "+nightly-2024-02-01-x86_64-unknown-linux-gnu",
            "wasm32-unknown-unknown" => "+nightly-2024-02-01-x86_64-unknown-linux-gnu",
            _ => "",
        };
        for add_feat in $add_feats {
            print_header $"workspace udeps - ($feat.name) / ($target) / ($add_feat)";
            let command = $'cargo ($toolchain) udeps --all-targets --target="($target)" --features="($feat.name), ($add_feat)"';
            print $"execute command: ($command)";
            nu -c $command;
        }
    }
}

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
