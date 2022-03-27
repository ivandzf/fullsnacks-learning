use std::collections::HashMap;

pub struct InMemory<T> {
    pub data: HashMap<String, T>,
}

pub trait InMemoryStore<T> {
    fn new() -> Self;
    fn get(&self, key: &str) -> Option<&T>;
    fn get_all(&self) -> &HashMap<String, T>;
    fn set(&mut self, key: String, value: T);
}

impl<T> InMemoryStore<T> for InMemory<T> {
    fn new() -> Self {
        InMemory {
            data: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key)
    }

    fn get_all(&self) -> &HashMap<String, T> {
        &self.data
    }

    fn set(&mut self, key: String, value: T) {
        self.data.insert(key, value);
    }
}
