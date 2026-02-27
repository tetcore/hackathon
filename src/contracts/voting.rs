use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tetcore_primitives::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voting {
    pub question: String,
    pub votes_for: u128,
    pub votes_against: u128,
    pub voters: HashMap<Address, bool>,
    pub ended: bool,
}

impl Voting {
    pub fn new(question: String) -> Self {
        Self {
            question,
            votes_for: 0,
            votes_against: 0,
            voters: HashMap::new(),
            ended: false,
        }
    }

    pub fn vote(&mut self, voter: Address, support: bool) -> Result<(), VotingError> {
        if self.ended {
            return Err(VotingError::VotingEnded);
        }

        if self.voters.contains_key(&voter) {
            return Err(VotingError::AlreadyVoted);
        }

        if support {
            self.votes_for += 1;
        } else {
            self.votes_against += 1;
        }

        self.voters.insert(voter, support);
        Ok(())
    }

    pub fn end(&mut self) -> (u128, u128) {
        self.ended = true;
        (self.votes_for, self.votes_against)
    }

    pub fn result(&self) -> String {
        if self.votes_for > self.votes_against {
            "Passed".to_string()
        } else if self.votes_against > self.votes_for {
            "Rejected".to_string()
        } else {
            "Tied".to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VotingError {
    VotingEnded,
    AlreadyVoted,
}

impl std::fmt::Display for VotingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VotingError::VotingEnded => write!(f, "Voting has ended"),
            VotingError::AlreadyVoted => write!(f, "Already voted"),
        }
    }
}

impl std::error::Error for VotingError {}
