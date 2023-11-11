Компоненты для построения системы сбора данных

## Компоненты

### Опрос и управление устройствами

#### [rsiot-modbus-client](./rsiot-modbus-client/README.md) - Modbus client (TCP, RTU)

![](./doc/component-modbus-client.svg)

#### OPC UA

#### S7 (контроллеры Сименс)

### Отдача данных

#### [rsiot-websocket-server](./rsiot-websocket-server/README.md) - Websocket Server

![](./doc/component-websocket-server.svg)

#### HTTP API

#### MQTT

#### Modbus TCP master

#### Telegram bot - отправка сообщений

### Брокеры сообщений

#### [rsiot-redis-publisher](./rsiot-redis-publisher/README.md) - публикация сообщений в Redis

![](./doc/component-redis-publisher.svg)

#### [rsiot-redis-subscriber](./rsiot-redis-subscriber/README.md) - получение данных из Redis

![](./doc/component-redis-subscriber.svg)

### Сохранение данных в базе

#### [rsiot-timescaledb-storing](./rsiot-timescaledb-storing/README.md) - TimescaleDB

![](./doc/component-timescaledb-storing.svg)

### Построение интерфейса

#### leptos

### Служебные компоненты

#### component-cache

Сохраняет все поступающие сообщения в коллекции, и передает исходное сообещение на выход

![](./doc/component-cache.svg)

#### component_combine

#### component_filter

#### component_mpsc_to_broadcast

#### component_mpsc_to_many_mpsc

## Описание

Отдельные компоненты выполнены в виде асинхронных задач `tokio`. Взаимодействие через очереди сообщений `tokio::sync::mpsc`.
