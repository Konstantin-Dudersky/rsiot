//! Преобразование метрик TokioRuntimeMetrics в строки для базы данных

use time::OffsetDateTime;

use crate::executor::TokioRuntimeMetrics;

use super::{ConfigTableField, ConfigTableFieldType, Error, row_with_ts};

impl TokioRuntimeMetrics {
    /// Конфигурация полей базы данных
    pub fn tsdb_fields() -> Vec<ConfigTableField> {
        vec![
            ConfigTableField {
                field_name: "workers_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "total_park_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "max_park_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "min_park_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "total_busy_duration".to_string(),
                data_type: ConfigTableFieldType::NumericDoublePrecision,
            },
            ConfigTableField {
                field_name: "max_busy_duration".to_string(),
                data_type: ConfigTableFieldType::NumericDoublePrecision,
            },
            ConfigTableField {
                field_name: "min_busy_duration".to_string(),
                data_type: ConfigTableFieldType::NumericDoublePrecision,
            },
            ConfigTableField {
                field_name: "global_queue_depth".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "mean_poll_duration".to_string(),
                data_type: ConfigTableFieldType::NumericDoublePrecision,
            },
            ConfigTableField {
                field_name: "mean_poll_duration_worker_min".to_string(),
                data_type: ConfigTableFieldType::NumericDoublePrecision,
            },
            ConfigTableField {
                field_name: "mean_poll_duration_worker_max".to_string(),
                data_type: ConfigTableFieldType::NumericDoublePrecision,
            },
            ConfigTableField {
                field_name: "total_noop_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "max_noop_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "min_noop_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "total_steal_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "max_steal_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "min_steal_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "total_steal_operations".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "max_steal_operations".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "min_steal_operations".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "num_remote_schedules".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "total_local_schedule_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "max_local_schedule_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "min_local_schedule_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "total_overflow_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "max_overflow_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "min_overflow_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "total_polls_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "max_polls_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "min_polls_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "total_local_queue_depth".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "max_local_queue_depth".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "min_local_queue_depth".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "blocking_queue_depth".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "live_tasks_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "blocking_threads_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "idle_blocking_threads_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "budget_forced_yield_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "io_driver_ready_count".to_string(),
                data_type: ConfigTableFieldType::NumericInteger,
            },
            ConfigTableField {
                field_name: "busy_ratio".to_string(),
                data_type: ConfigTableFieldType::NumericDoublePrecision,
            },
        ]
    }

    /// Преобразование метрик TokioRuntimeMetrics в строку для базы данных
    pub fn tsdb_row(self) -> Result<String, Error> {
        let time = OffsetDateTime::now_utc();

        row_with_ts(
            &time,
            &[
                self.workers_count.to_string(),
                self.total_park_count.to_string(),
                self.max_park_count.to_string(),
                self.min_park_count.to_string(),
                (self.total_busy_duration.as_nanos() as f64 / 1_000_000.0).to_string(),
                (self.max_busy_duration.as_nanos() as f64 / 1_000_000.0).to_string(),
                (self.min_busy_duration.as_nanos() as f64 / 1_000_000.0).to_string(),
                self.global_queue_depth.to_string(),
                (self.mean_poll_duration.as_nanos() as f64 / 1_000_000.0).to_string(),
                (self.mean_poll_duration_worker_min.as_nanos() as f64 / 1_000_000.0).to_string(),
                (self.mean_poll_duration_worker_max.as_nanos() as f64 / 1_000_000.0).to_string(),
                self.total_noop_count.to_string(),
                self.max_noop_count.to_string(),
                self.min_noop_count.to_string(),
                self.total_steal_count.to_string(),
                self.max_steal_count.to_string(),
                self.min_steal_count.to_string(),
                self.total_steal_operations.to_string(),
                self.max_steal_operations.to_string(),
                self.min_steal_operations.to_string(),
                self.num_remote_schedules.to_string(),
                self.total_local_schedule_count.to_string(),
                self.max_local_schedule_count.to_string(),
                self.min_local_schedule_count.to_string(),
                self.total_overflow_count.to_string(),
                self.max_overflow_count.to_string(),
                self.min_overflow_count.to_string(),
                self.total_polls_count.to_string(),
                self.max_polls_count.to_string(),
                self.min_polls_count.to_string(),
                self.total_local_queue_depth.to_string(),
                self.max_local_queue_depth.to_string(),
                self.min_local_queue_depth.to_string(),
                self.blocking_queue_depth.to_string(),
                self.live_tasks_count.to_string(),
                self.blocking_threads_count.to_string(),
                self.idle_blocking_threads_count.to_string(),
                self.budget_forced_yield_count.to_string(),
                self.io_driver_ready_count.to_string(),
                self.busy_ratio.to_string(),
            ],
        )
    }
}
