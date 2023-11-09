Компоненты для построения системы сбора данных

Опрос и управления устройствами:

- [x] Modbus client (TCP, RTU)

  - [rsiot-modbus-client](./rsiot-modbus-client/README.md)

- [ ] OPC UA client

- [ ] S7 (контроллеры Сименс)

Отдача данных:

- [ ] HTTP API

- [x] Websocket

  - [rsiot-websocket-server](./rsiot-websocket-server/README.md)

- [ ] MQTT

- [ ] Modbus TCP master

- [ ] Telegram bot - отправка сообщений

Передача сообщений через брокеры:

- [x] Redis

  - [rsiot-redis-publisher](./rsiot-redis-publisher/README.md) - публикация сообщений

  - [rsiot-redis-subscriber](./rsiot-redis-subscriber/README.md) - подписка на сообщения

Сохранение данных в БД:

- [x] TimescaleDB

  - [rsiot-timescaledb-storing](./rsiot-timescaledb-storing/README.md)

Построение интерфейса:

- [ ] leptos

Отдельные компоненты выполнены в виде асинхронных задач `tokio`. Взаимодействие через очереди сообщений `tokio::sync::mpsc`.
