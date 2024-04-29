use std::fmt::Binary;

pub struct State {
    state: u16,
}

impl State {
    pub fn new() -> Self {
        let state = Self { state: 0 };
        state
    }

    /// Переключить работу пина в режим входа
    pub fn set_input(&mut self, pin_index: usize) {
        self.set_output_low(pin_index);
    }

    /// Переключить работу пина в режим выхода в отключенном состоянии
    pub fn set_output_high(&mut self, pin: usize) {
        let mask = 1 << pin;
        self.state = self.state & !mask;
    }

    /// Переключить работу пина в режим выхода во включенном состоянии
    pub fn set_output_low(&mut self, pin: usize) {
        let mask = 1 << pin;
        self.state = self.state | mask;
    }

    /// Вернуть конфигурацию в виде двух байт
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
    #[test]
    fn test1() {
        let mut state = State::new();

        state.set_input(3);
        assert_eq!(state.state, 8);

        assert_eq!(state.to_bytes(), [8, 0]);

        state.set_output_high(3);
        assert_eq!(state.state, 0);
    }
}
