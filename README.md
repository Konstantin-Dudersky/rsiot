<!-- cargo-rdme start -->

Компоненты для построения системы сбора, обработки и визуализации данных

[Документация](https://konstantin-dudersky.github.io/rsiot-docs/)

## Обзор

![](./rsiot/doc/Новая%20концепция-2024-01-03-10-46.svg)


## Описание

**Компоненты** представляют собой асинхронные функции. У всех функций три аргумента:

```rust
async fn component<TMessage, TConfig>(
    input: Option<tokio::sync::mpsc::Receiver<TMessage>>,
    output: Option<tokio::sync::mpsc::Sender<TMessage>>,
    config: TConfig,
) -> ()
where
    TMessage: IMessage
{}
```

Сообщения между компонентами передаются через каналы "many producers to a single consumer"
библиотеки `tokio`.

Входной или выходной потоки могут быть не заданы, поэтому каналы обернуты в Option.

Структура конфигурации типа `TConfig` у каждого компонента своя.

Компоненты ничего не возвращают (точнее, возвращают тип `()`). Если в компоненте возникает
ошибка, логику перезапуска необходимо реализовать внутри данной функции. TODO - пересмотреть,
возможно стоит возвращать Result при критических ошибках.

**Сообщения** представляют собой тип enum, например:

```rust
use rsiot_messages_core::eav::EavModel;
use rsiot_messages_core::IMessage;
use serde::{Deserialize, Serialize};

[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    /// Текущее значение температуры
    Temperature(f64),
    /// Задание уставки
    ChangeSetpoint(f64),
}

impl IMessage for Message {
    fn into_eav(self) -> Vec<EavModel> {
        vec![]
    }}
```

Трейт `IMessage` реализует основные методы - см. документацию по крейту
[rsiot-messages-core](https://docs.rs/rsiot-messages-core/latest)

Для упрощения компоненты можно создавать и объединять в **цепочку компонентов**.


- может генерировать сообщения как на основе входных сообщений
- может генерировать сообщения периодически

 ## Флаги `feature`:

<!-- cargo-rdme end -->
