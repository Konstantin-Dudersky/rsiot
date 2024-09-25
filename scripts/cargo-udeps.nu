use shared.nu print_header

let targets = open target_config.json
let targets = $targets.targets

for target in $targets {
    for feat in $target.features {
        for add_feat in $target.add_feat {
            print_header $"cargo udeps - ($target.name) / ($feat) / ($add_feat)"
            let command = $'cargo +nightly udeps --all-targets --target="($target.name)" --features="($feat), ($add_feat), logging"'
            print $"execute command: ($command)\n"
            nu -c $command
        }
    }
}