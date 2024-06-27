# Извлечь данные для генерирования документации

# Для работы нужно установить утилиты:
# cargo install extract_anchors

extract_anchors src ../rsiot-docs/src/rsiot
rm -rf ../rsiot-docs/src/rsiot-config-files
cp -r rsiot-config-files/ ../rsiot-docs/src/
