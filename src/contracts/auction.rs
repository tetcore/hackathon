use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tetcore_primitives::Address;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auction {
    pub item_id: String,
    pub highest_bid: u128,
    pub highest_bidder: Option<Address>,
    pub bids: HashMap<Address, u128>,
    pub ended: bool,
}

impl Auction {
    pub fn new(item_id: String) -> Self {
        Self {
            item_id,
            highest_bid: 0,
            highest_bidder: None,
            bids: HashMap::new(),
            ended: false,
        }
    }

    pub fn bid(&mut self, bidder: Address, amount: u128) -> Result<(), AuctionError> {
        if self.ended {
            return Err(AuctionError::AuctionEnded);
        }

        if amount <= self.highest_bid {
            return Err(AuctionError::BidTooLow);
        }

        self.bids.insert(bidder, amount);
        self.highest_bid = amount;
        self.highest_bidder = Some(bidder);

        Ok(())
    }

    pub fn end(&mut self) -> Option<(Address, u128)> {
        self.ended = true;
        self.highest_bidder.map(|bidder| (bidder, self.highest_bid))
    }

    pub fn get_highest_bid(&self) -> u128 {
        self.highest_bid
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuctionError {
    AuctionEnded,
    BidTooLow,
}

impl std::fmt::Display for AuctionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuctionError::AuctionEnded => write!(f, "Auction has ended"),
            AuctionError::BidTooLow => write!(f, "Bid is too low"),
        }
    }
}

impl std::error::Error for AuctionError {}
