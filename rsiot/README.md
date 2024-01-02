<!-- cargo-rdme start -->

Компоненты для построения системы сбора данных
[Документация](https://docs.rs/rsiot/latest/rsiot)

## Компоненты

#### Взаимодействие с устройствами нижнего уровня

[**modbus-client**](https://docs.rs/rsiot-modbus-client/latest/rsiot_modbus_client/cmp_modbus_client)

Взаимодейтсвие с устройствами, поддерживающими протокол Modbus TCP сервер / Modbus RTU slave.

[**http-client**](https://docs.rs/rsiot-http-client/latest/rsiot_http_client/cmp_http_client)

Взаимодействие с устройствами, имеющими HTTP API.

[**websocket-client**](https://docs.rs/rsiot-websocket-client/latest/rsiot_websocket_client/cmp_websocket_client)

Взаимодействие с устройствами, поддерживющими функциональность Websocket сервера.

TODO **opcua-client**

Взаимодействие с контроллерами, имеющими функциональность OPC UA сервера.

TODO **s7-client**

Взаимодействие с контроллерами Siemens по протоколу S7.

[**esp-gpio**](https://docs.rs/rsiot-esp-gpio/latest/rsiot_esp_gpio/cmp_esp_gpio)

Чтение / запись данных с пинов GPIO микроконтроллера ESP.

#### Взаимодействие с системами верхнего уровня

[**http-server**](https://docs.rs/rsiot-http-server/latest/rsiot_http_server/cmp_http_server)

Поддержка HTTP API, через который внешние клиенты могут получать и вводить данные.

[**websocket-server**](https://docs.rs/rsiot-websocket-server/latest/rsiot_websocket_server/cmp_websocket_server)

Поддержка Websocket сервера, к которому могут подключаться внешние клиенты.

TODO **telegram**

#### Брокеры сообщений

[**redis-client**](https://docs.rs/rsiot-redis-client/latest/rsiot_redis_client/cmp_redis_client)

Подписка и публикация сообщения в Redis.

TODO **mqtt**

#### Сохранение данных в БД

[**timescaledb-storing**](https://docs.rs/rsiot-timescaledb-storing/latest/rsiot_timescaledb_storing/cmp_timescaledb_storing)

Сохрание сообщений в виде временных рядов в TimescaleDB.

#### Интерфейсы пользователя

TODO **leptos**

#### Вспомогательные крейты

[**plc**](https://docs.rs/rsiot-plc/latest)

Выполнение произвольной логики в "стиле PLC".

[**env-vars**](https://docs.rs/rsiot-env-vars/latest)

Чтение конфигурации из файла `.env`.

[**logging**](https://docs.rs/rsiot-logging/latest)

Настройка логгирования

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

Трейт `IMessage` реализует основные методы - см. документацию по крейту
[rsiot-messages-core](https://docs.rs/rsiot-messages-core/latest)

Для упрощения компоненты можно создавать и объединять в **цепочку компонентов**.

TODO - компонент для симуляции

- может генерировать сообщения как на основе входных сообщений
- может генерировать сообщения периодически

 ## Флаги `feature`:
TODO Написать про роутинг сообщений

<!-- cargo-rdme end -->
