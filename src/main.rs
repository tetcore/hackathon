use clap::{Parser, Subcommand};
use hackathon::{AIModel, Auction, Counter, HelloWorld, SimpleStorage, Token, Voting};

#[derive(Parser)]
#[command(name = "hackathon")]
#[command(version = "0.1.0")]
#[command(about = "Tetcore Hackathon - Build AI-powered blockchain projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hello {
        name: String,
    },
    Token {
        action: String,
        from: String,
        to: String,
        amount: u128,
    },
    Counter {
        action: String,
    },
    Auction {
        action: String,
        item: String,
        bidder: String,
        amount: u128,
    },
    Voting {
        action: String,
        question: String,
        voter: String,
        support: bool,
    },
    Storage {
        action: String,
        key: String,
        value: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hello { name } => {
            let hello = HelloWorld::new();
            println!("{}", hello.greet(&name));
        }

        Commands::Token {
            action,
            from,
            to,
            amount,
        } => {
            let mut token = Token::new(
                "Hackathon Token".to_string(),
                "HACK".to_string(),
                18,
                1_000_000,
            );
            println!("Token: {} ({})", token.name, token.symbol);
            println!("Initial supply: {}", token.total_supply);
            println!(
                "Action: {} - from {} to {} amount {}",
                action, from, to, amount
            );
        }

        Commands::Counter { action } => {
            let mut counter = Counter::new();
            match action.as_str() {
                "increment" => {
                    counter.increment();
                }
                "decrement" => {
                    counter.decrement();
                }
                "reset" => {
                    counter.reset();
                }
                _ => {}
            }
            println!("Counter: {}", counter.get());
        }

        Commands::Auction {
            action,
            item,
            bidder,
            amount,
        } => {
            let mut auction = Auction::new(item);
            println!("Auction for: {}", auction.item_id);
            println!("Action: {} - bidder {} amount {}", action, bidder, amount);
        }

        Commands::Voting {
            action,
            question,
            voter,
            support,
        } => {
            let mut voting = Voting::new(question);
            println!("Voting: {}", voting.question);
            println!("Action: {} - voter {} support {}", action, voter, support);
        }

        Commands::Storage { action, key, value } => {
            let mut storage = SimpleStorage::new();
            match action.as_str() {
                "set" => {
                    if let Some(v) = value {
                        storage.set(key.clone(), v.clone().into_bytes());
                        println!("Set {} = {}", key, v);
                    }
                }
                "get" => {
                    let v = storage.get(&key);
                    if let Some(data) = v {
                        println!("{} = {}", key, String::from_utf8_lossy(&data));
                    }
                }
                _ => {}
            }
        }
    }
}
