use chrono::{DateTime, Utc};
use diesel::Queryable;
use diesel::Insertable;
use diesel::AsChangeset;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, AsChangeset, Clone, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::task)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub completed: Option<bool>,
}

impl Task {
    pub fn new(id: String, description: String) -> Self {
        Task {
            id,
            description,
            completed: Some(false),
        }
    }
}

