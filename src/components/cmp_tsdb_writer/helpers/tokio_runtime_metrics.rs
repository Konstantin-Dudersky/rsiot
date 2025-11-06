//! Преобразование метрик TokioRuntimeMetrics в строки для базы данных

use time::OffsetDateTime;

use crate::executor::TokioRuntimeMetrics;

use super::{Error, RowBuilder};

impl TokioRuntimeMetrics {
    /// Преобразование метрик TokioRuntimeMetrics в строки для базы данных
    pub fn into_rows(
        self,
        prj: impl Into<String>,
        hst: impl Into<String>,
        svc: impl Into<String>,
    ) -> Result<Vec<String>, Error> {
        let builder = RowBuilder::prj(prj.into())
            .hst(hst.into())
            .svc(svc.into())
            .cmp("inner_task");

        let time = OffsetDateTime::now_utc();

        let rows = vec![
            builder.row_with_ts(
                "tokio_runtime_workers_count",
                self.workers_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_total_park_count",
                self.total_park_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_max_park_count",
                self.max_park_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_min_park_count",
                self.min_park_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_total_busy_duration",
                self.total_busy_duration.as_nanos() as f64 / 1_000_000.0,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_max_busy_duration",
                self.max_busy_duration.as_nanos() as f64 / 1_000_000.0,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_min_busy_duration",
                self.min_busy_duration.as_nanos() as f64 / 1_000_000.0,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_global_queue_depth",
                self.global_queue_depth as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_mean_poll_duration",
                self.mean_poll_duration.as_nanos() as f64 / 1_000_000.0,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_mean_poll_duration_worker_min",
                self.mean_poll_duration_worker_min.as_nanos() as f64 / 1_000_000.0,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_mean_poll_duration_worker_max",
                self.mean_poll_duration_worker_max.as_nanos() as f64 / 1_000_000.0,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_total_noop_count",
                self.total_noop_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_max_noop_count",
                self.max_noop_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_min_noop_count",
                self.min_noop_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_total_steal_count",
                self.total_steal_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_max_steal_count",
                self.max_steal_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_min_steal_count",
                self.min_steal_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_total_steal_operations",
                self.total_steal_operations as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_max_steal_operations",
                self.max_steal_operations as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_min_steal_operations",
                self.min_steal_operations as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_num_remote_schedules",
                self.num_remote_schedules as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_total_local_schedule_count",
                self.total_local_schedule_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_max_local_schedule_count",
                self.max_local_schedule_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_min_local_schedule_count",
                self.min_local_schedule_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_total_overflow_count",
                self.total_overflow_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_max_overflow_count",
                self.max_overflow_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_min_overflow_count",
                self.min_overflow_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_total_polls_count",
                self.total_polls_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_max_polls_count",
                self.max_polls_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_min_polls_count",
                self.min_polls_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_total_local_queue_depth",
                self.total_local_queue_depth as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_max_local_queue_depth",
                self.max_local_queue_depth as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_min_local_queue_depth",
                self.min_local_queue_depth as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_blocking_queue_depth",
                self.blocking_queue_depth as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_live_tasks_count",
                self.live_tasks_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_blocking_threads_count",
                self.blocking_threads_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_idle_blocking_threads_count",
                self.idle_blocking_threads_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_budget_forced_yield_count",
                self.budget_forced_yield_count as f64,
                &time,
            )?,
            builder.row_with_ts(
                "tokio_runtime_io_driver_ready_count",
                self.io_driver_ready_count as f64,
                &time,
            )?,
            builder.row_with_ts("tokio_runtime_busy_ratio", self.busy_ratio, &time)?,
        ];

        Ok(rows)
    }
}
