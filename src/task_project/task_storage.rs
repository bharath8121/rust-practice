use chrono::{DateTime, Utc};
use super::task::Task;

struct StorageError {
    error: String,
}

pub trait TaskStorage {
    fn store_task(task: &Task) -> Result<bool, StorageError>;
    fn delete_task(task: &Task) -> Result<bool, StorageError>;
    fn get_task_for_date(date: &DateTime<Utc>) -> Result<Task, StorageError>;
}

struct JsonStorage {
}

impl TaskStorage for JsonStorage {
    fn store_task(task: &Task) -> Result<bool, StorageError> {
        Ok(true)
    }
    fn delete_task(task: &Task) -> Result<bool, StorageError> {
        Ok(true)
    }
    fn get_task_for_date(date: &DateTime<Utc>) -> Result<Task, StorageError> {
        Ok(Task::new(String::from(""), String::from(""), true))
    }
}

pub fn new_task_storage() -> Box<JsonStorage> {
    Box::new(JsonStorage {})
}