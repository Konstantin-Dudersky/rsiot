```sh
cargo run --example cmp_linux_can --features="cmp_linux_can, log_console"

cargo build --release --target="armv7-unknown-linux-gnueabihf" --example cmp_linux_can --features="cmp_linux_can, log_console"; scp target/armv7-unknown-linux-gnueabihf/release/examples/cmp_linux_can root@target:~
```

```sh
sudo ip link add dev vcan0 type vcan
sudo ip link set up vcan0
```


canbusload vcan0@1000000
candump vcan0
cansniffer vcan0


ip -details -statistics link show can0


ip link set can0 down
ip link set can0 type can bitrate 1000000 dbitrate 1000000 fd on
ip link set can0 up
