use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub is_complete: bool,
}

impl Task {
    pub fn new(id: String, description: String, date: DateTime<Utc>) -> Self {
        Task {
            id,
            description,
            is_complete: false,
        }
    }
}

