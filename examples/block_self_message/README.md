Компонент не должен получать сообщения, которые сам создал.

Должно появляться сообщение "Received GenerateMessage in 2"

Сообщение "Received GenerateMessage in 1", наоборот, не должно появляться.

```sh
cargo run --example block_self_message
```
