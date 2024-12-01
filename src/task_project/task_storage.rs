use chrono::{DateTime, Utc};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};
use std::path::Path;
use super::task::Task;

#[derive(Debug)]
pub struct StorageError {
    error: String,
}

pub trait TaskStorage {
    fn create_task(&self, task: &Task) -> Result<(), StorageError>;
    fn update_task(&self, task: &Task) -> Result<(), StorageError>;
    fn delete_task(&self, id: String) -> Result<(), StorageError>;
    fn get_task_for_date(&self, date: DateTime<Utc>) -> Result<Vec<Task>, StorageError>;
    fn get_task_for_id(&self, id: String) -> Result<Task, StorageError>;
}

pub struct JsonStorage {
    file_path: String,
}

impl JsonStorage {

    fn save_tasks(&self, tasks: Vec<Task>) -> Result<(), StorageError> {
        // let res = OpenOptions::new().write(true).append(false).open(&self.file_path);
        let res = File::create(&self.file_path);
        match res {
            Err(e) => Err(StorageError { error: e.to_string() }),
            Ok(mut file) => {
                let res = serde_json::to_writer_pretty(&mut file, &tasks);
                match res {
                    Err(e) => Err(StorageError { error: format!("failed to write data: {:?}", e) }),
                    Ok(_) => Ok(()),
                }
            }
        }
    }

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
    fn create_task(&self, task: &Task) -> Result<(), StorageError> {
        let res = self.load_tasks();
        match res {
            Err(error) => { Err(error) },
            Ok(mut tasks) => {
                tasks.push(task.clone());
                self.save_tasks(tasks)
            }
        }
    }

    fn update_task(&self, task: &Task) -> Result<(), StorageError> {
        let res = self.load_tasks();
        match res {
            Err(error) => {
                Err(StorageError {error: format!("{} {:?}", "error updating task: ".to_string(), error) })
            },
            Ok(mut tasks) => {
                for t in tasks.iter_mut() {
                    if task.id == t.id {
                        t.description = task.description.to_string();
                        t.is_complete = task.is_complete.clone();
                    }
                }
                self.save_tasks(tasks)
            }
        }
    }

    fn delete_task(&self, id: String) -> Result<(), StorageError> {
        let res = self.load_tasks();
        match res {
            Err(error) => { Err(error) },
            Ok(mut tasks) => {
                let mut indx = 0;
                for (i, t) in tasks.iter().enumerate() {
                    if t.id == id {
                        indx = i;
                        break;
                    }
                }

                tasks.remove(indx);
                println!("final tasks {:?}", tasks);
                self.save_tasks(tasks)
            }
        }
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
            // let res = file.write("[]".as_bytes());
            // match res {
            //     Ok(_) => {}
            //     Err(_) => {}
            // }
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