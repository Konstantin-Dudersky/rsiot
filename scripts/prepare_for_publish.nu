# Подготовка к публикации

use shared.nu print_header

let RSIOT_DOCS = "../rsiot-docs/src"
let RSIOT_DOCS_RUSTDOC = $"($RSIOT_DOCS)/rustdoc"
let RSIOT_DOCS_SRC = $"($RSIOT_DOCS)/src-rsiot"

let targets = open target_config.json
let targets = $targets.targets


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





# cargo test ---------------------------------------------------------------------------------------

# print_header "workspace test - x86_64-unknown-linux-gnu / multi-thread"
# cargo test --all-targets --target="x86_64-unknown-linux-gnu" --features=""
# TODO - тесты не проходят
