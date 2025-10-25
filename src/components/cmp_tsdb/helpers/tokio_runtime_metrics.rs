//! Преобразование метрик TokioRuntimeMetrics в строки для базы данных

use time::OffsetDateTime;

use crate::executor::TokioRuntimeMetrics;

use super::{Error, Row, RowBuilder};

impl TokioRuntimeMetrics {
    /// Преобразование метрик TokioRuntimeMetrics в строки для базы данных
    pub fn into_rows(
        self,
        hst: impl Into<String>,
        svc: impl Into<String>,
    ) -> Result<Vec<Row>, Error> {
        let builder = RowBuilder::new()
            .hst(hst.into())
            .svc(svc.into())
            .cmp("inner_task");

        let time = OffsetDateTime::now_utc();

        let rows = vec![
            builder
                .clone()
                .key("tokio_runtime_workers_count")
                .value(self.workers_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_total_park_count")
                .value(self.total_park_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_max_park_count")
                .value(self.max_park_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_min_park_count")
                .value(self.min_park_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_total_busy_duration")
                .value(self.total_busy_duration.as_nanos() as f64 / 1_000_000.0)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_max_busy_duration")
                .value(self.max_busy_duration.as_nanos() as f64 / 1_000_000.0)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_min_busy_duration")
                .value(self.min_busy_duration.as_nanos() as f64 / 1_000_000.0)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_global_queue_depth")
                .value(self.global_queue_depth as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_elapsed")
                .value(self.elapsed.as_secs_f64())
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_mean_poll_duration")
                .value(self.mean_poll_duration.as_nanos() as f64 / 1_000_000.0)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_mean_poll_duration_worker_min")
                .value(self.mean_poll_duration_worker_min.as_nanos() as f64 / 1_000_000.0)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_mean_poll_duration_worker_max")
                .value(self.mean_poll_duration_worker_max.as_nanos() as f64 / 1_000_000.0)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_total_noop_count")
                .value(self.total_noop_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_max_noop_count")
                .value(self.max_noop_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_min_noop_count")
                .value(self.min_noop_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_total_steal_count")
                .value(self.total_steal_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_max_steal_count")
                .value(self.max_steal_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_min_steal_count")
                .value(self.min_steal_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_total_steal_operations")
                .value(self.total_steal_operations as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_max_steal_operations")
                .value(self.max_steal_operations as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_min_steal_operations")
                .value(self.min_steal_operations as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_num_remote_schedules")
                .value(self.num_remote_schedules as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_total_local_schedule_count")
                .value(self.total_local_schedule_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_max_local_schedule_count")
                .value(self.max_local_schedule_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_min_local_schedule_count")
                .value(self.min_local_schedule_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_total_overflow_count")
                .value(self.total_overflow_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_max_overflow_count")
                .value(self.max_overflow_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_min_overflow_count")
                .value(self.min_overflow_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_total_polls_count")
                .value(self.total_polls_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_max_polls_count")
                .value(self.max_polls_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_min_polls_count")
                .value(self.min_polls_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_total_local_queue_depth")
                .value(self.total_local_queue_depth as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_max_local_queue_depth")
                .value(self.max_local_queue_depth as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_min_local_queue_depth")
                .value(self.min_local_queue_depth as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_blocking_queue_depth")
                .value(self.blocking_queue_depth as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_live_tasks_count")
                .value(self.live_tasks_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_blocking_threads_count")
                .value(self.blocking_threads_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_idle_blocking_threads_count")
                .value(self.idle_blocking_threads_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_budget_forced_yield_count")
                .value(self.budget_forced_yield_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_io_driver_ready_count")
                .value(self.io_driver_ready_count as f64)
                .time(time)
                .row()?,
            builder
                .clone()
                .key("tokio_runtime_busy_ratio")
                .value(self.busy_ratio)
                .time(time)
                .row()?,
        ];

        Ok(rows)
    }
}
