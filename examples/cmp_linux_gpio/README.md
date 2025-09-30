```sh
cargo build --release --target="armv7-unknown-linux-gnueabihf" --example cmp_linux_gpio --features="cmp_linux_gpio, log_console"; scp target/armv7-unknown-linux-gnueabihf/release/examples/cmp_linux_gpio root@target:~
```
