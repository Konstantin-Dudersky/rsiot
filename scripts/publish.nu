# публикация пакетов, пока cargo ws publish выдает ошибку SSL

do -i {
    cargo publish --allow-dirty -p rsiot-messages-core
}
do -i {
    cargo publish --allow-dirty -p rsiot-component-core
}
do -i {
    cargo publish --allow-dirty -p rsiot-components-config
}
do -i {
    cargo publish --allow-dirty -p rsiot-env-vars
}
do -i {
    cargo publish --allow-dirty -p rsiot-extra-components
}
do -i {
    cargo publish --allow-dirty -p rsiot-http-client
}
do -i {
    cargo publish --allow-dirty -p rsiot-http-server
}
do -i {
    cargo publish --allow-dirty -p rsiot-logging
}
do -i {
    cargo publish --allow-dirty -p rsiot-modbus-client
}
do -i {
    cargo publish --allow-dirty -p rsiot-redis-publisher
}
do -i {
    cargo publish --allow-dirty -p rsiot-redis-subscriber
}
do -i {
    cargo publish --allow-dirty -p rsiot-timescaledb-storing
}
do -i {
    cargo publish --allow-dirty -p rsiot-websocket-client
}
do -i {
    cargo publish --allow-dirty -p rsiot-websocket-server
}
do -i {
    cargo publish --allow-dirty -p rsiot
}