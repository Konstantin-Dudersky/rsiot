use std::{fmt::Binary, sync::Arc};

use tokio::sync::Mutex;

#[derive(Clone)]
pub struct State {
    state: Arc<Mutex<u16>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(0)),
        }
    }

    /// Переключить работу пина в режим входа
    pub async fn set_input(&mut self, pin_index: usize) {
        self.set_output_low(pin_index).await;
    }

    /// Переключить работу пина в режим выхода в отключенном состоянии
    pub async fn set_output_high(&mut self, pin: usize) {
        let mut state = self.state.lock().await;
        let mask = 1 << pin;
        *state &= !mask;
    }

    /// Переключить работу пина в режим выхода во включенном состоянии
    pub async fn set_output_low(&mut self, pin: usize) {
        let mut state = self.state.lock().await;
        let mask = 1 << pin;
        *state |= mask;
    }

    /// Вернуть конфигурацию в виде двух байт
    pub async fn to_bytes(&self) -> [u8; 2] {
        self.state.lock().await.to_le_bytes()
    }
}

impl Binary for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = *self.state.blocking_lock();
        write!(f, "{:b}", state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Запуск:
    ///
    /// ```bash
    /// cargo test --target x86_64-unknown-linux-gnu --lib --features executor -- drivers_i2c::pcf8575::state::tests::test1 --exact --show-output
    /// ```
    #[tokio::test]
    async fn test1() {
        let mut state = State::new();

        state.set_input(3).await;
        assert_eq!(*state.state.lock().await, 8);

        assert_eq!(state.to_bytes().await, [8, 0]);

        state.set_output_high(3).await;
        assert_eq!(*state.state.lock().await, 0);
    }
}
