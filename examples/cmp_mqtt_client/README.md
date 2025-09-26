Запустить NanoMq

```sh
docker run --name nanomq -p 1883:1883 -p 8083:8083 -p 8883:8883 emqx/nanomq:latest
```


Запустить публикацию
```sh
cargo run --example mqtt_client_publisher --features "cmp_mqtt_client, serde_json, serde_cbor" --target="x86_64-unknown-linux-gnu"
```

Запустить публикацию (ESP)
```sh
cargo run --example esp_mqtt_client_publisher --features "cmp_esp, serde_json, serde_cbor" --target="riscv32imc-esp-espidf"
```

Запустить подписку
```sh
cargo run --example mqtt_client_subscriber --features "cmp_mqtt_client, serde_json, serde_cbor" --target="x86_64-unknown-linux-gnu"
```

Запустить подписку (ESP)
```sh
cargo run --example esp_mqtt_client_subscriber --features "cmp_esp, serde_json, serde_cbor" --target="riscv32imc-esp-espidf"
```

Удалить контейнер NanoMQ

```sh
docker rm -f nanomq
```
