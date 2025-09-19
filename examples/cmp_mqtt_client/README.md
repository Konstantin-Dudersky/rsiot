Запустить NanoMq

```sh
docker run -d --name nanomq -p 1883:1883 -p 8083:8083 -p 8883:8883 emqx/nanomq:latest
```

```sh
cargo run --example cmp_mqtt_client --features "cmp_mqtt_client, serde_json" --target="x86_64-unknown-linux-gnu"
```


Удалить контейнер NanoMQ

```sh
docker rm -f nanomq
```
