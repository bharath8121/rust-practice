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
    fn remove_task(&self, task_id: String) -> Result<bool, TaskManagerError>;
    fn move_task(&self, task_id: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError>;
    fn edit_description(&self, task_id: String, description: String) -> Result<Task, TaskManagerError>;
}

pub struct TaskManagerImpl {
    storage: Box<dyn TaskStorage>,
}

impl TaskManager for TaskManagerImpl {

    fn add_task(&self, description: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError> {
        let id = uuid::Uuid::new_v4();
        let task = Task::new(id.to_string(), description);
        let res = self.storage.create_task(&task);
        match res {
            Ok(_) => Ok(task),
            Err(err) => Err(TaskManagerError { message: String::from(format!("Failed to add task: {:?}", err) ) }),
        }
    }

    // need to fix this
    fn finish_task(&self, id: String) -> Result<(), TaskManagerError> {
        let res = self.storage.get_task_for_id(id);
        let mut task = match res {
            Ok(task) => task,
            Err(err) => return Err(TaskManagerError { message: String::from(format!("Failed to finish task: {:?}", err) ) }),
        };

        task.is_complete = true;
        let res = self.storage.update_task(&task);
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(TaskManagerError { message: String::from(format!("Failed to finish task: {:?}", err) ) }),
        }
    }

    fn remove_task(&self, id: String) -> Result<bool, TaskManagerError> {
        let res = self.storage.delete_task(id);
        match res {
            Ok(_) => Ok(true),
            Err(err) => Err(TaskManagerError { message: String::from(format!("Failed to remove task: {:?}", err) ) }),
        }
    }

    fn move_task(&self, task_id: String, date: DateTime<Utc>) -> Result<Task, TaskManagerError> {
        let res = self.storage.get_task_for_id(task_id);
        let mut task = match res {
            Ok(task) => task,
            Err(err) => return Err(TaskManagerError { message: String::from(format!("Failed to move task: {:?}", err) ) }),
        };

        // task.date = date;
        let res = self.storage.update_task(&task);
        match res {
            Ok(_) => Ok(task),
            Err(err) => Err(TaskManagerError { message: String::from(format!("Failed to move task: {:?}", err) ) }),
        }
    }

    fn edit_description(&self, task_id: String, description: String) -> Result<Task, TaskManagerError> {
        let res = self.storage.get_task_for_id(task_id);
        let mut task = match res {
            Ok(task) => task,
            Err(err) => return Err(TaskManagerError { message: String::from(format!("Failed to edit task: {:?}", err) ) }),
        };

        task.description = description;
        let res = self.storage.update_task(&task);
        match res {
            Ok(_) => Ok(task),
            Err(err) => Err(TaskManagerError { message: String::from(format!("Failed to edit task: {:?}", err) ) }),
        }
    }
}

pub fn new_task_manager() -> Box<dyn TaskManager> {
    Box::new(TaskManagerImpl{
        storage: new_json_task_storage("".to_string()),
    })
}