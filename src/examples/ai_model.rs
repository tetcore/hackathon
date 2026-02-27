use serde::{Deserialize, Serialize};
use tetcore_primitives::{Address, Hash32};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    pub model_id: Hash32,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub input_format: String,
    pub output_format: String,
    pub price_per_call: u64,
    pub is_active: bool,
}

impl AIModel {
    pub fn new(name: String, description: String, price_per_call: u64) -> Self {
        Self {
            model_id: Hash32::empty(),
            owner: Address::from_bytes([0u8; 32]),
            name,
            description,
            input_format: "text".to_string(),
            output_format: "text".to_string(),
            price_per_call,
            is_active: true,
        }
    }

    pub fn infer(&self, _input: &str) -> Result<String, HackathonError> {
        if !self.is_active {
            return Err(HackathonError::ModelNotActive);
        }
        Ok(format!("Processed by {}", self.name))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HackathonError {
    ModelNotActive,
    InsufficientPayment,
    InvalidInput,
}

impl std::fmt::Display for HackathonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HackathonError::ModelNotActive => write!(f, "Model is not active"),
            HackathonError::InsufficientPayment => write!(f, "Insufficient payment"),
            HackathonError::InvalidInput => write!(f, "Invalid input"),
        }
    }
}

impl std::error::Error for HackathonError {}
