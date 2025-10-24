//! Преобразование метрик TokioRuntimeMetrics в строки для базы данных

use crate::executor::TokioRuntimeMetrics;

use super::Row;

const ENTITY: &str = "tokio_runtime";

impl From<&TokioRuntimeMetrics> for Vec<Row> {
    fn from(value: &TokioRuntimeMetrics) -> Self {
        vec![
            Row::new_simple(ENTITY, "workers_count", value.workers_count as f64),
            Row::new_simple(ENTITY, "total_park_count", value.total_park_count as f64),
            Row::new_simple(ENTITY, "max_park_count", value.max_park_count as f64),
            Row::new_simple(ENTITY, "min_park_count", value.min_park_count as f64),
            Row::new_simple(
                ENTITY,
                "total_busy_duration",
                value.total_busy_duration.as_nanos() as f64 / 1_000_000.0,
            ),
            Row::new_simple(
                ENTITY,
                "max_busy_duration",
                value.max_busy_duration.as_nanos() as f64 / 1_000_000.0,
            ),
            Row::new_simple(
                ENTITY,
                "min_busy_duration",
                value.min_busy_duration.as_nanos() as f64 / 1_000_000.0,
            ),
            Row::new_simple(
                ENTITY,
                "global_queue_depth",
                value.global_queue_depth as f64,
            ),
            Row::new_simple(ENTITY, "elapsed", value.elapsed.as_secs_f64()),
            Row::new_simple(
                ENTITY,
                "mean_poll_duration",
                value.mean_poll_duration.as_nanos() as f64 / 1_000_000.0,
            ),
            Row::new_simple(
                ENTITY,
                "mean_poll_duration_worker_min",
                value.mean_poll_duration_worker_min.as_nanos() as f64 / 1_000_000.0,
            ),
            Row::new_simple(
                ENTITY,
                "mean_poll_duration_worker_max",
                value.mean_poll_duration_worker_max.as_nanos() as f64 / 1_000_000.0,
            ),
            Row::new_simple(ENTITY, "total_noop_count", value.total_noop_count as f64),
            Row::new_simple(ENTITY, "max_noop_count", value.max_noop_count as f64),
            Row::new_simple(ENTITY, "min_noop_count", value.min_noop_count as f64),
            Row::new_simple(ENTITY, "total_steal_count", value.total_steal_count as f64),
            Row::new_simple(ENTITY, "max_steal_count", value.max_steal_count as f64),
            Row::new_simple(ENTITY, "min_steal_count", value.min_steal_count as f64),
            Row::new_simple(
                ENTITY,
                "total_steal_operations",
                value.total_steal_operations as f64,
            ),
            Row::new_simple(
                ENTITY,
                "max_steal_operations",
                value.max_steal_operations as f64,
            ),
            Row::new_simple(
                ENTITY,
                "min_steal_operations",
                value.min_steal_operations as f64,
            ),
            Row::new_simple(
                ENTITY,
                "num_remote_schedules",
                value.num_remote_schedules as f64,
            ),
            Row::new_simple(
                ENTITY,
                "total_local_schedule_count",
                value.total_local_schedule_count as f64,
            ),
            Row::new_simple(
                ENTITY,
                "max_local_schedule_count",
                value.max_local_schedule_count as f64,
            ),
            Row::new_simple(
                ENTITY,
                "min_local_schedule_count",
                value.min_local_schedule_count as f64,
            ),
            Row::new_simple(
                ENTITY,
                "total_overflow_count",
                value.total_overflow_count as f64,
            ),
            Row::new_simple(
                ENTITY,
                "max_overflow_count",
                value.max_overflow_count as f64,
            ),
            Row::new_simple(
                ENTITY,
                "min_overflow_count",
                value.min_overflow_count as f64,
            ),
            Row::new_simple(ENTITY, "total_polls_count", value.total_polls_count as f64),
            Row::new_simple(ENTITY, "max_polls_count", value.max_polls_count as f64),
            Row::new_simple(ENTITY, "min_polls_count", value.min_polls_count as f64),
            Row::new_simple(
                ENTITY,
                "total_local_queue_depth",
                value.total_local_queue_depth as f64,
            ),
            Row::new_simple(
                ENTITY,
                "max_local_queue_depth",
                value.max_local_queue_depth as f64,
            ),
            Row::new_simple(
                ENTITY,
                "min_local_queue_depth",
                value.min_local_queue_depth as f64,
            ),
            Row::new_simple(
                ENTITY,
                "blocking_queue_depth",
                value.blocking_queue_depth as f64,
            ),
            Row::new_simple(ENTITY, "live_tasks_count", value.live_tasks_count as f64),
            Row::new_simple(
                ENTITY,
                "blocking_threads_count",
                value.blocking_threads_count as f64,
            ),
            Row::new_simple(
                ENTITY,
                "idle_blocking_threads_count",
                value.idle_blocking_threads_count as f64,
            ),
            Row::new_simple(
                ENTITY,
                "budget_forced_yield_count",
                value.budget_forced_yield_count as f64,
            ),
            Row::new_simple(
                ENTITY,
                "io_driver_ready_count",
                value.io_driver_ready_count as f64,
            ),
            Row::new_simple(ENTITY, "busy_ratio", value.busy_ratio),
        ]
    }
}
