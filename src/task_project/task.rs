#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub is_complete: bool,
}

impl Task {
    pub fn new(id: String, description: String, is_complete: bool) -> Self {
        Task {
            id,
            description,
            is_complete
        }
    }
}

