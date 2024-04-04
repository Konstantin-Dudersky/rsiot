# Подготовка к публикации

def print_header [header: string] {
    print $"\n\n(ansi magenta_bold)($header)(ansi reset)\n\n"
}

cd rsiot

let targets = [
    {
        name: "x86_64-unknown-linux-gnu",
        toolchain: "+stable",
        features: [
            "cmp_auth",
            "cmp_http_client",
            "cmp_http_server",
            "cmp_influxdb",
            "cmp_modbus_client",
            "cmp_plc",
            "cmp_redis_client",
            "cmp_slint",
            "cmp_surrealdb",
            "cmp_timescaledb",
            "cmp_websocket_client",
            "cmp_websocket_server",
            "env_vars",
            "executor",
            "logging",
        ],
        add_feat: ["", "single-thread"],
    },
    {
        name: "aarch64-linux-android",
        toolchain: "+stable",
        features: [
            "cmp_http_client",
            "cmp_slint",
            "executor",
            "logging",
        ],
        add_feat: ["", "single-thread"],
    },
    {
        name: "aarch64-unknown-linux-gnu",
        toolchain: "+stable",
        features: [
            "cmp_auth",
            "cmp_http_client",
            "cmp_http_server",
            "cmp_influxdb",
            "cmp_modbus_client",
            "cmp_plc",
            "cmp_redis_client",
            "cmp_slint",
            "cmp_surrealdb",
            "cmp_timescaledb",
            "cmp_websocket_client",
            "cmp_websocket_server",
            "env_vars",
            "executor",
            "logging",
        ],
        add_feat: ["", "single-thread"],
    },
    {
        name: "riscv32imc-esp-espidf",
        # TODO - https://github.com/rust-lang/libc/issues/3615
        toolchain: "+nightly-2024-02-01-x86_64-unknown-linux-gnu",
        features: [
            "cmp_esp",
            "cmp_plc",
            "executor",
        ],
        add_feat: ["single-thread"],
    },
    {
        name: "wasm32-unknown-unknown",
        toolchain: "+stable",
        features: [
            "cmp_http_client_wasm",
            "cmp_leptos",
            "cmp_plc",
            "cmp_websocket_client_wasm",
            "cmp_webstorage",
            "executor",
            "logging",
        ],
        add_feat: ["single-thread"],
    },
]

# cargo clippy -------------------------------------------------------------------------------------

for target in $targets {
    for feat in $target.features {
        for add_feat in $target.add_feat {
            print_header $"workspace clippy - ($target.name) / ($feat) / ($add_feat)"
            let command = $'cargo ($target.toolchain) clippy --all-targets --target="($target.name)" --features="($feat), ($add_feat), logging"'
            print $"execute command: ($command)\n"
            nu -c $command
        }
    }
}

# cargo doc ----------------------------------------------------------------------------------------

rm -rf ../../rsiot-docs/src/rustdoc

for target in $targets {
    print_header $"cargo doc - ($target.name)"
    
    # create folder
    let command = $"mkdir ../../rsiot-docs/src/rustdoc/($target.name)"
    nu -c $command;
    
    # combine features
    let features = $target.features | append $target.add_feat | str join ', '

    # generate doc
    let command = $'cargo doc --target ($target.name) --features="($features), logging"  --no-deps --document-private-items -Zunstable-options -Zrustdoc-scrape-examples'
    print $"execute command: ($command)\n"
    nu -c $command;

    # copy files
    let command = $"cp -r target/($target.name)/doc/* ../../rsiot-docs/src/rustdoc/($target.name)"
    nu -c $command;
}

return; # TODO - доделать

# cargo udeps --------------------------------------------------------------------------------------

let features = [];
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
