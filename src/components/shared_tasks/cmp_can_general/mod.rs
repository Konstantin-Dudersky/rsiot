//! Запуск общих задач компонентов cmp_linux_can и cmp_esp_can

mod can_general_tasks;
mod task_input;
mod task_output;
mod task_periodic;

pub use can_general_tasks::CanGeneralTasks;
