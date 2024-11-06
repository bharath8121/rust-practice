use chrono::{DateTime, Utc};
use super::task::Task;

#[derive(Debug)]
pub struct StorageError {
    error: String,
}

pub trait TaskStorage {
    fn store_task(&self, task: &Task) -> Result<bool, StorageError>;
    fn delete_task(&self, task: &Task) -> Result<bool, StorageError>;
    fn get_task_for_date(&self, date: DateTime<Utc>) -> Result<Vec<Task>, StorageError>;
    fn get_task_for_id(&self, id: String) -> Result<Task, StorageError>;
}

pub struct JsonStorage {
}

impl TaskStorage for JsonStorage {
    fn store_task(&self, task: &Task) -> Result<bool, StorageError> {
        println!("received: {}", task.id);
        Ok(true)
    }
    fn delete_task(&self, task: &Task) -> Result<bool, StorageError> {
        Ok(true)
    }
    fn get_task_for_date(&self, date: DateTime<Utc>) -> Result<Vec<Task>, StorageError> {
        todo!()
    }

    fn get_task_for_id(&self, id: String) -> Result<Task, StorageError> {
        todo!()
    }
}

pub fn new_json_task_storage() -> Box<dyn TaskStorage> {
    Box::new(JsonStorage{})
}