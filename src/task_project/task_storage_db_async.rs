use std::sync::OnceLock;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait, QueryFilter};
use tokio::runtime::Runtime;
use crate::configuration::configuration::Configuration;
use crate::task_project::task_storage::{StorageError};
use crate::entities::prelude::*;
use crate::entities::task;
use crate::task_project::task::Task as TT;

pub struct TaskStorageDBAsync {
}

static ONCE: OnceLock<DatabaseConnection> = OnceLock::new();


impl TaskStorageDBAsync {
    pub async fn create_task(&self, t: task::ActiveModel) -> Result<(), StorageError> {
        let conn = self.get_db_instance().await;
        match conn {
            Ok(conn) => {
                t.insert(conn).await
                    .map_err(|e| StorageError{error: e.to_string() })
                    .and_then(|_| Ok(()))
            },
            Err(e) => {
                Err(e)
            }
        }
    }

    pub async fn update_task(&self, t: task::ActiveModel) -> Result<(), StorageError> {
        let conn = self.get_db_instance().await;
        match conn {
            Ok(conn) => {
                t.update(conn).await
                    .map_err(|e| StorageError{error: "error updating data".to_string() })
                    .and_then(|_| Ok(()))
            },
            Err(e) => {
                Err(e)
            }
        }

    }

    pub async fn delete_task(&self, id: String) -> Result<(), StorageError> {
        todo!()
    }

    pub async fn get_task_for_date(&self, date: DateTime<Utc>) -> Result<Vec<Task>, StorageError> {
        todo!()
    }

    pub async fn get_task_for_id(&self, id: String) -> Result<Option<TT>, StorageError> {
        let conn = self.get_db_instance().await;
        match conn {
            Ok(conn) => {
                task::Entity::find().filter(task::Column::Id.eq(id)).one(conn).await
                    .map_err(|e| StorageError{error: e.to_string()})
                    .and_then(|o| {
                        return if o.is_some() {
                            let t = o.unwrap();
                            Ok(Some(TT {
                                id: t.id,
                                description: t.description,
                                completed: t.completed,
                            }))
                        } else {
                            Err(StorageError { error: "error".to_string() })
                        }
                    })
            },
            Err(e) => {
                Err(e)
            }
        }

    }


    async fn get_db_instance(&self) -> Result<&'static DatabaseConnection, StorageError> {
        // ONCE.get_or_init(|| {
        //     let rt = Runtime::new().expect("Error creating runtime");
        //     rt.block_on(async {
        //         let db_url = Configuration::get_db_url();
        //         let mut opt = ConnectOptions::new(db_url);
        //         opt.max_connections(50)
        //             .min_connections(2)
        //             .sqlx_logging(true);
        //         Database::connect(opt.clone()).await.expect("Error creating database")
        //     })
        // })
        let conn = ONCE.get();
        if let Some(conn) = conn {
            return Ok(conn);
        }

        let db_url = Configuration::get_db_url();
        let mut opt = ConnectOptions::new(db_url);
        opt.max_connections(50)
            .min_connections(2)
            .sqlx_logging(true);
        let res = Database::connect(opt.clone()).await;
        if res.is_err() {
            return Err(StorageError{error: res.unwrap_err().to_string()});
        }

        let conn = res.unwrap();
        let res = ONCE.set(conn);
        if res.is_err() {
            return Err(StorageError{error: "error inserting data into singleton".to_string() });
        }

        let conn = ONCE.get();
        if let Some(conn) = conn {
            return Ok(conn);
        }
        Err(StorageError{error: "error getting data".to_string() })
    }
}