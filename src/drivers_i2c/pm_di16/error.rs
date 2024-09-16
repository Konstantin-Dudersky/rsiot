use crate::components::shared_tasks;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    TaskFilterIdenticalData(shared_tasks::filter_identical_data::Error),

    #[error(transparent)]
    TaskMpscToMsgBus(shared_tasks::mpsc_to_msg_bus::Error),
}
