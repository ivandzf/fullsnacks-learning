use std::collections::HashMap;

pub struct InMemory {
    pub data: HashMap<String, String>,
}

pub trait InMemoryStore {
    fn new() -> Self;
    fn get(&self, key: &str) -> Option<&String>;
    fn get_all(&self) -> &HashMap<String, String>;
    fn set(&mut self, key: String, value: String);
}

impl InMemoryStore for InMemory {
    fn new() -> Self {
        InMemory {
            data: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn get_all(&self) -> &HashMap<String, String> {
        &self.data
    }

    fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
}
