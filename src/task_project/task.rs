use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub is_complete: bool,
}

impl Task {
    pub fn new(id: String, description: String, date: DateTime<Utc>) -> Self {
        Task {
            id,
            description,
            date,
            is_complete: false,
        }
    }
}

