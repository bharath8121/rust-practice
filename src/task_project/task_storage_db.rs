use chrono::{DateTime, Utc};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::configuration::configuration::Configuration;
use crate::task_project::task::Task;
use crate::task_project::task_storage::{StorageError, TaskStorage};
use diesel::prelude::*;
use diesel::row::NamedRow;
use crate::schema::task::dsl::*;

pub struct TaskStorageDB {
    pool_manager: Pool<ConnectionManager<PgConnection>>,
}

impl TaskStorage for TaskStorageDB {
    fn create_task(&self, t: &Task) -> Result<(), StorageError> {
        let mut conn = self.pool_manager.get();

        // improve error
        if conn.is_err() {
            return Err(StorageError{error: "unable to get connection from pool".to_string()});
        }

        diesel::insert_into(task).values(t).execute(&mut conn.unwrap())
            .map(|_| ())
            .map_err(|e| StorageError{error: "unable to insert data. err:".to_string() + &e.to_string() })

    }

    fn update_task(&self, t: &Task) -> Result<(), StorageError> {
        let mut conn = self.pool_manager.get();

        if conn.is_err() {
            return Err(StorageError{error: "unable to get connection from pool".to_string()});
        }

        diesel::update(task.filter(id.eq(&t.id))).set(t).execute(&mut conn.unwrap())
            .map(|_| ())
            .map_err(|e| StorageError{error: "unable to update data. err".to_string() + &e.to_string()})
    }

    fn delete_task(&self, t: String) -> Result<(), StorageError> {
        todo!()
    }

    fn get_task_for_date(&self, date: DateTime<Utc>) -> Result<Vec<Task>, StorageError> {
        todo!()
    }

    fn get_task_for_id(&self, i: String) -> Result<Task, StorageError> {
        let mut conn = self.pool_manager.get();

        if conn.is_err() {
            return Err(StorageError{error: "unable to get connection from pool".to_string()});
        }

        task.filter(id.eq(&i)).first::<Task>(&mut conn.unwrap())
            .map_err(|e| StorageError{error: "unable to get data. err:".to_string() + &e.to_string()})
    }
}

impl TaskStorageDB {
    pub fn new() -> Box<dyn TaskStorage> {
        let manager = ConnectionManager::<PgConnection>::new(Configuration::get_db_url());
        Box::new(TaskStorageDB {
            pool_manager: Pool::builder().max_size(10)
                .min_idle(Some(2))
                .build(manager)
                .expect("Failed to create pool."),
        })
    }
}


