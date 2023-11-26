Компоненты для построения системы сбора данных

## Компоненты

#### Взаимодействие с устройствами нижнего уровня

[**modbus-client**](https://docs.rs/rsiot-modbus-client/latest/)

Взаимодейтсвие с устройствами, поддерживающими протокол Modbus TCP сервер / Modbus RTU slave.

[**http-client**](https://docs.rs/rsiot-http-client/latest/)

Взаимодействие с устройствами, имеющих HTTP API.

[**websocket-client**](https://docs.rs/rsiot-websocket-client/latest/)

Взаимодействие с устройствами, поддерживющими функциональность Websocket сервера.

TODO **opcua-client**

Взаимодействие с контроллерами, имеющими функциональность OPC UA сервера.

TODO **s7-client**

Взаимодействие с контроллерами Siemens по протоколу S7.

#### Взаимодействие с системами верхнего уровня

[**http-server**](https://docs.rs/rsiot-http-server/latest/)

Поддержка HTTP API, через который внешние клиенты могут получать и вводить данные.

[**websocket-server**](https://docs.rs/rsiot-websocket-server/latest/)

Поддержка Websocket сервера, к которому могут подключаться внешние клиенты.

TODO **mqtt**

TODO **telegram**

#### Брокеры сообщений

[**redis-publisher**](https://docs.rs/rsiot-redis-publisher/latest/)

Публикация сообщений в Redis.

[**redis-subscriber**](https://docs.rs/rsiot-redis-subscriber/latest/)

Подписка на сообщения из Redis.

#### Сохранение данных в БД

[**timescaledb-storing**](https://docs.rs/rsiot-timescaledb-storing/latest)

Сохрание сообщений в виде временных рядов в TimescaleDB.

#### Интерфейсы пользователя

TODO **leptos**

## Описание

**Компоненты** представляют собой асинхронные функции. У всех функций три аргумента:

```rust
# use tokio;
# use rsiot_messages_core::IMessage;
async fn component<TMessage, TConfig>(
    input: Option<tokio::sync::mpsc::Receiver<TMessage>>,
    output: Option<tokio::sync::mpsc::Sender<TMessage>>,
    config: TConfig,
) -> ()
where
    TMessage: IMessage
{}
```

Сообщения между компонентами передаются через каналы "many producers to a single consumer" библиотеки `tokio`.

Входной или выходной потоки могут быть не заданы, поэтому каналы обернуты в Option.

Структура конфигурации типа `TConfig` у каждого компонента своя.

Компоненты ничего не возвращают (точнее, возвращают тип `()`). Если в компоненте возникает ошибка, логику перезапуска необходимо реализовать внутри данной функции.
TODO - пересмотреть, возможно стоит возвращать Result при критических ошибках.

**Сообщения** представляют собой тип enum, например:

```rust
use rsiot_messages_core::IMessage;
use serde::{Deserialize, Serialize};
[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    /// Текущее значение температуры
    Temperature(f64),
    /// Задание уставки
    ChangeSetpoint(f64),
}

impl IMessage for Message {}
```

Трейт `IMessage` реализует основные методы - см. документацию по крейту [rsiot-messages-core](https://docs.rs/rsiot-messages-core/latest)

Для упрощения компоненты можно создавать и объединять в **цепочку компонентов**.

TODO - добавить пример с Modbus-клиентом
