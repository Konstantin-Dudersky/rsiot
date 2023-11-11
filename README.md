Компоненты для построения системы сбора данных

## Компоненты

### Опрос и управление устройствами

#### [x] [rsiot-modbus-client](./rsiot-modbus-client/README.md) - Modbus client (TCP, RTU)

![](./doc/component-modbus-client.svg)

[README](./rsiot-modbus-client/README.md)

#### [ ] OPC UA

#### [ ] S7 (контроллеры Сименс)

### Отдача данных

#### [x] rsiot-websocket-server - Websocket Server

![](./doc/component-websocket-server.svg)

[README](./rsiot-websocket-server/README.md)

#### [ ] HTTP API

#### [ ] MQTT

#### [ ] Modbus TCP master

#### [ ] Telegram bot - отправка сообщений

### Брокеры сообщений

#### [x] rsiot-redis-publisher - публикация сообщений в Redis

![](./doc/component-redis-publisher.svg)

[README](./rsiot-redis-publisher/README.md)

#### [x] rsiot-redis-subscriber - получение данных из Redis

![](./doc/component-redis-subscriber.svg)

[README](./rsiot-redis-subscriber/README.md)

### Сохранение данных в базе

#### [x] rsiot-timescaledb-storing - TimescaleDB

[README](./rsiot-timescaledb-storing/README.md)

### Построение интерфейса

#### [ ] leptos

## Описание

Отдельные компоненты выполнены в виде асинхронных задач `tokio`. Взаимодействие через очереди сообщений `tokio::sync::mpsc`.
