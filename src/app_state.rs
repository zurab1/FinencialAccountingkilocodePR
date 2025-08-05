use crate::database::Database;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<Database>,
}

impl AppState {
    pub fn new(database: Database) -> Self {
        Self {
            database: Arc::new(database),
        }
    }
}
