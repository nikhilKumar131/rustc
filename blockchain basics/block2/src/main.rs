use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u32,
    timestamp: DateTime<Utc>,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u32,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::from("11111"),
            nonce: 0,
        };
        println!("working here");
        block.mine();
        block
    }

    fn calculate_hash(index: u32, timestamp: &DateTime<Utc>, data: &str, previous_hash: &str, nonce: u32) -> String {
        let input = format!("{}-{:?}-{}-{}-{}", index, timestamp, data, previous_hash, nonce);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        hex::encode(result)
    }

    fn mine(&mut self) {
        while !self.is_valid_hash() {
            self.nonce += 1;
            self.hash = Block::calculate_hash(
                self.index,
                &self.timestamp,
                &self.data,
                &self.previous_hash,
                self.nonce,
            );
        }
    }

    fn is_valid_hash(&self) -> bool {
        &self.hash[0..4] == "0000" // Example difficulty requirement: leading four zeros
    }
}

fn main() {
    let mut previous_hash = "0".to_owned();
    let mut index = 0;

    loop {
        println!("Enter data for Block {}: (type 'exit' to stop)", index);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let data = input.trim().to_owned();

        if data == "exit" {
            break;
        }

        let block = Block::new(index, data.clone(), previous_hash.clone());
        println!("{:?}", block);

        previous_hash = block.hash;
        index += 1;
    }
}
