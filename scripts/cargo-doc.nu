use shared.nu print_header

let targets = open target_config.json
let targets = $targets.targets

for target in $targets {
    let features = $"($target.features)" | str trim --char '[' | str trim --char ']'
    let features = $"($features), rustdoc"
    let add_feat = $target.add_feat.0
    print_header $"cargo doc - ($target.name) / ($features) / ($add_feat)"

    # generate docs
    let command = $'cargo ($target.toolchain) doc --target ($target.name) --features="($features), ($add_feat)" --no-deps'
    print $"execute command: ($command)\n"
    nu -c $command

    # delete docs
    let dir = $"../rsiot-docs/($target.name)"
    let command = $"rm -rf ($dir)"
    nu -c $command

    # create dir
    let command = $"mkdir ($dir)"
    nu -c $command;

    # copy files
    let command = $"cp -r target/($target.name)/doc/* ($dir)"
    nu -c $command;
}
