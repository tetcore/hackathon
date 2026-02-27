use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelloWorld {
    pub message: String,
}

impl HelloWorld {
    pub fn new() -> Self {
        Self {
            message: "Hello, Tetcore!".to_string(),
        }
    }

    pub fn greet(&self, name: &str) -> String {
        format!("{}, {}!", self.message, name)
    }
}

impl Default for HelloWorld {
    fn default() -> Self {
        Self::new()
    }
}
