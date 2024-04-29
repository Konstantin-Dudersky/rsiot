use std::{fmt::Binary, sync::Arc};

use tokio::sync::Mutex;

pub struct State {
    state: u16,
}

impl State {
    pub fn new() -> Arc<Mutex<Self>> {
        let state = Self { state: 0 };
        Arc::new(Mutex::new(state))
    }

    pub fn set_input(&mut self, pin: u8) {
        self.set_output_low(pin);
    }

    pub fn set_output_high(&mut self, pin: u8) {
        let mask = 1 << pin;
        self.state = self.state & !mask;
    }

    pub fn set_output_low(&mut self, pin: u8) {
        let mask = 1 << pin;
        self.state = self.state | mask;
    }

    pub fn to_bytes(&self) -> [u8; 2] {
        self.state.to_le_bytes()
    }
}

impl Binary for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.state)
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
        let state = State::new();
        let mut state = state.lock().await;

        state.set_input(3);
        assert_eq!(state.state, 8);

        assert_eq!(state.to_bytes(), [8, 0]);

        state.set_output_high(3);
        assert_eq!(state.state, 0);
    }
}
