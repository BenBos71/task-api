use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::task::Task;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub tasks: Arc<Mutex<HashMap<Uuid, Task>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}