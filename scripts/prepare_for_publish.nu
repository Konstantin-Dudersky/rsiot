# Подготовка к публикации

# TODO - https://github.com/rust-lang/libc/issues/3615
# проблема со сборкой ESP в новых версиях, перейти на найтли, когда пофиксят

let RSIOT_DOCS = "../rsiot-docs/src"
let RSIOT_DOCS_RUSTDOC = $"($RSIOT_DOCS)/rustdoc"
let RSIOT_DOCS_SRC = $"($RSIOT_DOCS)/src-rsiot"

def print_header [header: string] {
    print $"\n\n(ansi magenta_bold)($header)(ansi reset)\n\n"
}

let targets = open target_config.json
let targets = $targets.targets

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

# delete files
let command = $"rm -rf ($RSIOT_DOCS_RUSTDOC)"
nu -c $command

for target in $targets {
    print_header $"cargo doc - ($target.name)"
    
    # create folder
    let command = $"mkdir ($RSIOT_DOCS_RUSTDOC)/($target.name)"
    nu -c $command;
    
    # combine features
    let features = $target.features | append $target.add_feat | str join ', '

    # generate doc
    let command = $'cargo +nightly-x86_64-unknown-linux-gnu doc --target ($target.name) --features="($features), logging"  --no-deps --document-private-items -Zunstable-options -Zrustdoc-scrape-examples'
    print $"execute command: ($command)\n"
    nu -c $command;

    # copy doc files
    let command = $"cp -r target/($target.name)/doc/* ($RSIOT_DOCS_RUSTDOC)/($target.name)"
    nu -c $command;
}

# copy src files to mdbook -------------------------------------------------------------------------
print_header "Copy src files to mdbook"

let command = $"rm -rf ($RSIOT_DOCS_SRC)"
nu -c $command;
let command = $"mkdir ($RSIOT_DOCS_SRC)"
nu -c $command;
let command = $"cp -r src/* ($RSIOT_DOCS_SRC)"
nu -c $command;

# cargo rdme ---------------------------------------------------------------------------------------

do {
    print_header "readme"
    cargo rdme --force
}

# cargo update -------------------------------------------------------------------------------------

print_header "workspace - update"
cargo update

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




# cargo outdated -----------------------------------------------------------------------------------

print_header "workspace - outdated"
cargo outdated


# cargo test ---------------------------------------------------------------------------------------

# print_header "workspace test - x86_64-unknown-linux-gnu / multi-thread"
# cargo test --all-targets --target="x86_64-unknown-linux-gnu" --features=""
# TODO - тесты не проходят

