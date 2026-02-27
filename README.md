# Tetcore Hackathon

A hackathon crate for building AI-powered blockchain projects on Tetcore.

## Overview

This crate provides examples and utilities for the Tetcore Hackathon. It includes:

- **Examples**: Simple contracts demonstrating core concepts
- **Contracts**: More complex smart contracts
- **CLI**: Command-line interface for testing

## Getting Started

```bash
# Build
cargo build

# Run examples
cargo run -- hello Alice
cargo run -- token transfer 100
cargo run -- counter increment
cargo run -- storage set mykey myvalue
```

## Examples

### Hello World

```rust
use tetcore_hackathon::HelloWorld;

let hello = HelloWorld::new();
println!("{}", hello.greet("Alice"));  // "Hello, Tetcore! Alice!"
```

### AI Model

```rust
use tetcore_hackathon::AIModel;

let model = AIModel::new(
    "Sentiment Analyzer".to_string(),
    "Analyzes text sentiment".to_string(),
    100,  // price per call
);
let result = model.infer("I love Tetcore!").unwrap();
```

### Token

```rust
use tetcore_hackathon::Token;

let mut token = Token::new(
    "My Token".to_string(),
    "MTK".to_string(),
    18,           // decimals
    1_000_000,   // initial supply
);
token.transfer(&from, &to, 100).unwrap();
```

## Contracts

### Counter

```rust
use tetcore_hackathon::Counter;

let mut counter = Counter::new();
counter.increment();
counter.increment();
assert_eq!(counter.get(), 2);
```

### Auction

```rust
use tetcore_hackathon::Auction;

let mut auction = Auction::new("NFT #1".to_string());
auction.bid(bidder, 1000).unwrap();
```

### Voting

```rust
use tetcore_hackathon::Voting;

let mut voting = Voting::new("Should we upgrade?".to_string());
voting.vote(voter1, true).unwrap();
voting.vote(voter2, false).unwrap();
println!("Result: {}", voting.result());
```

## CLI Usage

```bash
# Hello world
tetcore-hackathon hello --name Alice

# Token operations
tetcore-hackathon token --action transfer --from alice --to bob --amount 100

# Counter
tetcore-hackathon counter --action increment

# Auction
tetcore-hackathon auction --action bid --item "NFT #1" --bidder alice --amount 100

# Voting
tetcore-hackathon voting --action vote --question "Upgrade?" --voter alice --support true

# Storage
tetcore-hackathon storage --action set --key "name" --value "Tetcore"
tetcore-hackathon storage --action get --key "name"
```

## Dependencies

- `tetcore-primitives` - Core types (Address, Hash, etc.)
- `tetcore-kernel` - Protocol kernel
- `tetcore-runtime` - Runtime modules
- `tetcore-vm` - TVM and TCL

## License

MIT
