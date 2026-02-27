use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tetcore_primitives::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u128,
    pub balances: HashMap<Address, u128>,
}

impl Token {
    pub fn new(name: String, symbol: String, decimals: u8, initial_supply: u128) -> Self {
        let mut balances = HashMap::new();
        let genesis = Address::from_bytes([0u8; 32]);
        balances.insert(genesis, initial_supply);

        Self {
            name,
            symbol,
            decimals,
            total_supply: initial_supply,
            balances,
        }
    }

    pub fn transfer(
        &mut self,
        from: &Address,
        to: &Address,
        amount: u128,
    ) -> Result<(), TokenError> {
        let from_balance = self.balances.get(from).unwrap_or(&0);

        if *from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }

        let from_new = from_balance - amount;
        let to_balance = self.balances.get(to).unwrap_or(&0);
        let to_new = to_balance + amount;

        self.balances.insert(*from, from_new);
        self.balances.insert(*to, to_new);

        Ok(())
    }

    pub fn balance_of(&self, account: &Address) -> u128 {
        *self.balances.get(account).unwrap_or(&0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenError {
    InsufficientBalance,
    InvalidAmount,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::InsufficientBalance => write!(f, "Insufficient balance"),
            TokenError::InvalidAmount => write!(f, "Invalid amount"),
        }
    }
}

impl std::error::Error for TokenError {}
