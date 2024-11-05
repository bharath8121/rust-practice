use chrono::{DateTime, Utc};

use super::task::Task;
use super::task_storage::{new_json_task_storage, TaskStorage};

#[derive(Debug)]
pub struct TaskManagerError {
    message: String,
}

pub trait TaskManager {
    fn add_task(&self, description: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError>;
    fn finish_task(&self, id: String) -> Result<(), TaskManagerError>;
    fn get_all_tasks(&self) -> Result<Vec<Task>, TaskManagerError>;
    fn remove_task(&self, task_id: String) -> Result<Task, TaskManagerError>;
    fn move_task(&self, task_id: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError>;
    fn edit_task(&self, task_id: String, description: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError>;
}

pub struct TaskManagerImpl {
    storage: Box<dyn TaskStorage>,
}

impl TaskManager for TaskManagerImpl {

    fn add_task(&self, description: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError> {
        let id = uuid::Uuid::new_v4();
        let task = Task::new(id.to_string(), description, date);
        let res = self.storage.store_task(&task);
        match res {
            Ok(_) => Ok(task),
            Err(err) => Err(TaskManagerError { message: String::from(format!("Failed to add task: {:?}", err) ) }),
        }
    }

    fn finish_task(&self, id: String) -> Result<(), TaskManagerError> {
        todo!()
    }

    fn get_all_tasks(&self) -> Result<Vec<Task>, TaskManagerError> {
        todo!()
    }

    fn remove_task(&self, task_id: String) -> Result<Task, TaskManagerError> {
        todo!()
    }

    fn move_task(&self, task_id: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError> {
        todo!()
    }

    fn edit_task(&self, task_id: String, description: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError> {
        todo!()
    }

}

pub fn new_task_manager() -> Box<dyn TaskManager> {
    Box::new(TaskManagerImpl{
        storage: new_json_task_storage(),
    })
}