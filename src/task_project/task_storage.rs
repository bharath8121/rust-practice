use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Error};
use std::mem;
use std::mem::{drop};
use std::ops::Deref;
use super::task::Task;

#[derive(Debug)]
pub struct StorageError {
    error: String,
}

pub trait TaskStorage {
    fn create_task(&self, task: &Task) -> Result<bool, StorageError>;
    fn update_task(&self, task: &Task) -> Result<(), StorageError>;
    fn delete_task(&self, id: String) -> Result<bool, StorageError>;
    fn get_task_for_date(&self, date: DateTime<Utc>) -> Result<Vec<Task>, StorageError>;
    fn get_task_for_id(&self, id: String) -> Result<Task, StorageError>;
}

pub struct JsonStorage {
    file_path: String,
}

impl JsonStorage {

    fn _load_tasks(&self, file: File) -> Result<Vec<Task>, StorageError> {
        let reader = BufReader::new(file);
        let stream: Result<Vec<Task>, serde_json::Error> = serde_json::from_reader(reader);
        match stream {
            Err(error) => {
                Err(StorageError {error: "error getting tasks: ".to_string() + error.to_string().as_mut_str() })
            },
            Ok(stream) => Ok(stream),
        }
    }

    fn load_tasks(&self) -> Result<Vec<Task>, StorageError> {
        let res = File::open(&self.file_path);
        match res {
            Err(error) => {
                Err(StorageError {error: "error opening file".to_string() + error.to_string().as_mut_str() })
            },
            Ok(file) => {
                self._load_tasks(file)
            }
        }
    }
}

impl TaskStorage for JsonStorage {
    fn create_task(&self, task: &Task) -> Result<bool, StorageError> {
        println!("received: {}", task.id);
        Ok(true)
    }

    fn update_task(&self, task: &Task) -> Result<(), StorageError> {
        todo!()
    }

    fn delete_task(&self, id: String) -> Result<bool, StorageError> {
        Ok(true)
    }
    fn get_task_for_date(&self, date: DateTime<Utc>) -> Result<Vec<Task>, StorageError> {
        todo!()
    }

    fn get_task_for_id(&self, id: String) -> Result<Task, StorageError> {
        let res = self.load_tasks();
        match res {
            Err(error) => {
                Err(StorageError {error: format!("{} {:?}", "error loading tasks".to_string(), error)})
            },
            Ok(tasks) => {
                for task in tasks {
                    if task.id == id {
                        return Ok(task.clone());
                    }
                }
                Err(StorageError{error: format!("task with id {} not found", id) })
            }
        }
    }
}

pub fn new_json_task_storage(file_path: String) -> Box<dyn TaskStorage> {

    let res = OpenOptions::new()
        .create(true)
        .append(true).open(&file_path);

    match res {
        Ok(file) => {
            drop(file);
        },
        Err(e) => {
            panic!("error creating file: {}", e);
        }
    }

    Box::new(JsonStorage{
        file_path: String::from(file_path)
    })
}