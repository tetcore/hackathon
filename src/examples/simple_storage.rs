use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleStorage {
    pub data: HashMap<String, Vec<u8>>,
}

impl SimpleStorage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Vec<u8>) {
        self.data.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.data.get(key).cloned()
    }

    pub fn remove(&mut self, key: &str) -> Option<Vec<u8>> {
        self.data.remove(key)
    }

    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

impl Default for SimpleStorage {
    fn default() -> Self {
        Self::new()
    }
}
