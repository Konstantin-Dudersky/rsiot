use std::{fmt::Display, time::Duration};

/// Статистика выполнения SQL-запроса
#[derive(Default)]
pub struct QueryStat {
    /// Количество строк
    pub rows_count: usize,

    /// Размер строки SQL-запроса в байтах
    pub sql_string_len: usize,

    /// Время, прошедшее с момента последнего выполнения запроса
    pub last_execution: Duration,

    /// Время выполнения запроса
    pub execution_time: Duration,
}

impl QueryStat {
    /// Создать структуру со значениями по умолчанию
    pub fn new() -> Self {
        Default::default()
    }

    /// Установить количество строк
    pub fn set_rows_count(&mut self, rows_count: usize) {
        self.rows_count = rows_count;
    }

    /// Установить размер строки SQL-запроса в байтах
    pub fn set_sql_string_len(&mut self, sql_string_len: usize) {
        self.sql_string_len = sql_string_len;
    }

    /// Установить время последнего выполнения запроса
    pub fn set_last_execution(&mut self, last_execution: Duration) {
        self.last_execution = last_execution;
    }

    /// Установить время выполнения запроса
    pub fn set_execution_time(&mut self, execution_time: Duration) {
        self.execution_time = execution_time;
    }
}

impl Display for QueryStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SQL query stat: row count: {}; time elapsed: {} ms; SQL string len: {:.1} KB; execution time: {} ms",
            self.rows_count,
            self.last_execution.as_millis(),
            self.sql_string_len as f32 / 1024.0,
            self.execution_time.as_millis()
        )
    }
}
