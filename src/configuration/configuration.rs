use std::env;
use std::sync::{OnceLock};

pub struct Configuration {
}


impl Configuration {
    pub fn get_db_url() -> String {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    }
}






