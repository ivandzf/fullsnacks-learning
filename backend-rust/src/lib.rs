use std::{collections::HashMap, sync::{Mutex, Arc}};

pub struct AppState {
    pub store: InMemory,
}

pub struct InMemory {
    pub data: Arc<Mutex<HashMap<String, String>>>,
}

pub trait InMemoryStore {
    fn new() -> Self;
    fn get(&self, key: &str) -> Option<String>;
    fn get_all(&self) -> Vec<String>;
    fn add(&mut self, key: String, value: String);
}

impl InMemoryStore for InMemory {
    fn new() -> Self {
        InMemory {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().get(key).cloned()
    }

    fn get_all(&self) -> Vec<String> {
        self.data
            .lock()
            .unwrap()
            .values()
            .map(|v| v.to_string())
            .collect()
    }

    fn add(&mut self, key: String, value: String) {
        self.data.lock().unwrap().insert(key, value);
    }
}
