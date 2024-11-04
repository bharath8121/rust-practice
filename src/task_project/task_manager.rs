use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use uuid::{Uuid, Timestamp};

use super::task::Task;
use super::task_storage::{new_task_storage, TaskStorage};

#[derive(Debug)]
pub struct TaskManagerError {
    message: String,
}

pub trait TaskManager {
    fn add_task(&self, description: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError>;
    fn finish_task(&self, id: String) -> Result<(), TaskManagerError>;
}

pub struct TaskManagerImpl {
    storage: Box<dyn TaskStorage>,
}


impl TaskManager for TaskManagerImpl {
    fn add_task(&self, description: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError> {
        println!("Adding task desc: {}, date: {}", description, date);
        Ok(Task::new(String::from(Uuid::new_v4()), description, false))
    }
    fn finish_task(&self, id: String) -> Result<(), TaskManagerError> {
        println!("Finishing task: {}", id);
        Ok(())
    }
}

pub fn new_task_manager() -> Box<dyn TaskManager> {
    Box::new(TaskManagerImpl{
        storage: new_task_storage(),
    })
}