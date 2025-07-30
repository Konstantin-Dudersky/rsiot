use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Метрики tokio
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TokioRuntimeMetrics {
    /// The number of worker threads used by the runtime.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::workers_count`]
    pub workers_count: usize,

    /// The number of times worker threads parked.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::total_park_count`]
    pub total_park_count: u64,

    /// The maximum number of times any worker thread parked.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::max_park_count`]
    pub max_park_count: u64,

    /// The minimum number of times any worker thread parked.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::min_park_count`]
    pub min_park_count: u64,

    /// The amount of time worker threads were busy.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::total_busy_duration`]
    pub total_busy_duration: Duration,

    /// The maximum amount of time a worker thread was busy.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::max_busy_duration`]
    pub max_busy_duration: Duration,

    /// The minimum amount of time a worker thread was busy.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::min_busy_duration`]
    pub min_busy_duration: Duration,

    /// The number of tasks currently scheduled in the runtime's global queue.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::global_queue_depth`]
    pub global_queue_depth: usize,

    /// Total amount of time elapsed since observing runtime metrics.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::elapsed`]
    pub elapsed: Duration,

    /// The average duration of a single invocation of poll on a task.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::mean_poll_duration`]
    pub mean_poll_duration: Duration,

    /// The average duration of a single invocation of poll on a task on the worker with the lowest
    /// value.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::mean_poll_duration_worker_min`]
    pub mean_poll_duration_worker_min: Duration,

    /// The average duration of a single invocation of poll on a task on the worker with the highest
    /// value.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::mean_poll_duration_worker_max`]
    pub mean_poll_duration_worker_max: Duration,

    /// A histogram of task polls since the previous probe grouped by poll times.
    pub poll_time_histogram: Vec<u64>,

    /// The number of times worker threads unparked but performed no work before parking again.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::total_noop_count`]
    pub total_noop_count: u64,

    /// The maximum number of times any worker thread unparked but performed no work before parking
    /// again.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::max_noop_count`]
    pub max_noop_count: u64,

    /// The minimum number of times any worker thread unparked but performed no work before parking
    /// again.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::min_noop_count`]
    pub min_noop_count: u64,

    /// The number of tasks worker threads stole from another worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::total_steal_count`]
    pub total_steal_count: u64,

    /// The maximum number of tasks any worker thread stole from another worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::max_steal_count`]
    pub max_steal_count: u64,

    /// The minimum number of tasks any worker thread stole from another worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::min_steal_count`]
    pub min_steal_count: u64,

    /// The number of times worker threads stole tasks from another worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::total_steal_operations`]
    pub total_steal_operations: u64,

    /// The maximum number of times any worker thread stole tasks from another worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::max_steal_operations`]
    pub max_steal_operations: u64,

    /// The minimum number of times any worker thread stole tasks from another worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::min_steal_operations`]
    pub min_steal_operations: u64,

    /// The number of tasks scheduled from **outside** of the runtime.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::num_remote_schedules`]
    pub num_remote_schedules: u64,

    /// The number of tasks scheduled from worker threads.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::total_local_schedule_count`]
    pub total_local_schedule_count: u64,

    /// The maximum number of tasks scheduled from any one worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::max_local_schedule_count`]
    pub max_local_schedule_count: u64,

    /// The minimum number of tasks scheduled from any one worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::min_local_schedule_count`]
    pub min_local_schedule_count: u64,

    /// The number of times worker threads saturated their local queues.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::total_overflow_count`]
    pub total_overflow_count: u64,

    /// The maximum number of times any one worker saturated its local queue.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::max_overflow_count`]
    pub max_overflow_count: u64,

    /// The minimum number of times any one worker saturated its local queue.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::min_overflow_count`]
    pub min_overflow_count: u64,

    /// The number of tasks that have been polled across all worker threads.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::total_polls_count`]
    pub total_polls_count: u64,

    /// The maximum number of tasks that have been polled in any worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::max_polls_count`]
    pub max_polls_count: u64,

    /// The minimum number of tasks that have been polled in any worker thread.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::min_polls_count`]
    pub min_polls_count: u64,

    /// The total number of tasks currently scheduled in workers' local queues.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::total_local_queue_depth`]
    pub total_local_queue_depth: usize,

    /// The maximum number of tasks currently scheduled any worker's local queue.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::max_local_queue_depth`]
    pub max_local_queue_depth: usize,

    /// The minimum number of tasks currently scheduled any worker's local queue.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::min_local_queue_depth`]
    pub min_local_queue_depth: usize,

    /// The number of tasks currently waiting to be executed in the runtime's blocking threadpool.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::blocking_queue_depth`]
    pub blocking_queue_depth: usize,

    /// The current number of alive tasks in the runtime.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::live_tasks_count`]
    pub live_tasks_count: usize,

    /// The number of additional threads spawned by the runtime.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::blocking_threads_count`]
    pub blocking_threads_count: usize,

    /// The number of idle threads, which have spawned by the runtime for `spawn_blocking` calls.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::idle_blocking_threads_count`]
    pub idle_blocking_threads_count: usize,

    /// Returns the number of times that tasks have been forced to yield back to the scheduler after
    /// exhausting their task budgets.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::budget_forced_yield_count`]
    pub budget_forced_yield_count: u64,

    /// Returns the number of ready events processed by the runtime’s I/O driver.
    ///
    /// Ссылка: [`tokio_metrics::RuntimeMetrics::io_driver_ready_count`]
    pub io_driver_ready_count: u64,

    /// Returns the ratio of the `total_polls_count` to the `total_noop_count`.
    pub mean_polls_per_park: f64,

    /// Returns the ratio of the `total_busy_duration` to the `elapsed`.
    pub busy_ratio: f64,
}

#[cfg(feature = "log_tokio")]
impl From<tokio_metrics::RuntimeMetrics> for TokioRuntimeMetrics {
    fn from(value: tokio_metrics::RuntimeMetrics) -> Self {
        Self {
            workers_count: value.workers_count,
            total_park_count: value.total_park_count,
            max_park_count: value.max_park_count,
            min_park_count: value.min_park_count,
            total_busy_duration: value.total_busy_duration,
            max_busy_duration: value.max_busy_duration,
            min_busy_duration: value.min_busy_duration,
            global_queue_depth: value.global_queue_depth,
            elapsed: value.elapsed,
            mean_poll_duration: value.mean_poll_duration,
            mean_poll_duration_worker_min: value.mean_poll_duration_worker_min,
            mean_poll_duration_worker_max: value.mean_poll_duration_worker_max,
            poll_time_histogram: value.poll_time_histogram.clone(),
            total_noop_count: value.total_noop_count,
            max_noop_count: value.max_noop_count,
            min_noop_count: value.min_noop_count,
            total_steal_count: value.total_steal_count,
            max_steal_count: value.max_steal_count,
            min_steal_count: value.min_steal_count,
            total_steal_operations: value.total_steal_operations,
            max_steal_operations: value.max_steal_operations,
            min_steal_operations: value.min_steal_operations,
            num_remote_schedules: value.num_remote_schedules,
            total_local_schedule_count: value.total_local_schedule_count,
            max_local_schedule_count: value.max_local_schedule_count,
            min_local_schedule_count: value.min_local_schedule_count,
            total_overflow_count: value.total_overflow_count,
            max_overflow_count: value.max_overflow_count,
            min_overflow_count: value.min_overflow_count,
            total_polls_count: value.total_polls_count,
            max_polls_count: value.max_polls_count,
            min_polls_count: value.min_polls_count,
            total_local_queue_depth: value.total_local_queue_depth,
            max_local_queue_depth: value.max_local_queue_depth,
            min_local_queue_depth: value.min_local_queue_depth,
            blocking_queue_depth: value.blocking_queue_depth,
            live_tasks_count: value.live_tasks_count,
            blocking_threads_count: value.blocking_threads_count,
            idle_blocking_threads_count: value.idle_blocking_threads_count,
            budget_forced_yield_count: value.budget_forced_yield_count,
            io_driver_ready_count: value.io_driver_ready_count,
            mean_polls_per_park: value.mean_polls_per_park(),
            busy_ratio: value.busy_ratio(),
        }
    }
}
