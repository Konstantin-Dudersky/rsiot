cross build --target aarch64-unknown-linux-gnu --release;
scp target/aarch64-unknown-linux-gnu/release/test_slint user@target:/home/user/

sudo apt install libxkbcommon-x11-0

sudo apt install libinput10

sudo apt-get install libgbm-dev

## Запуск на целевом устройстве

```bash
SLINT_KMS_ROTATION=90 /home/user/test_slint
```
