use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counter {
    pub count: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn increment(&mut self) -> i32 {
        self.count += 1;
        self.count
    }

    pub fn decrement(&mut self) -> i32 {
        self.count -= 1;
        self.count
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }

    pub fn get(&self) -> i32 {
        self.count
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}
