use scripts/shared.nu print_header

let targets = open target_config.json
let targets = $targets.targets

for target in $targets {
    for feat in $target.features {
        for add_feat in $target.add_feat {
            print_header $"workspace outdated - ($target.name) / ($feat) / ($add_feat)"
            let command = $'cargo ($target.toolchain) outdated --features="($feat), ($add_feat), logging"'
            print $"execute command: ($command)\n"
            nu -c $command
        }
    }
}
