use chrono::Utc;
use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(timestamp, &data, &previous_hash);
        Block {
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(timestamp: i64, data: &str, previous_hash: &str) -> String {
        let input = format!("{}-{}-{}", timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        hex::encode(result)
    }
}

fn main() {
    // Create the genesis block
    let genesis_block = Block::new("Genesis Block".to_owned(), "0".to_owned());
    println!("{:?}", genesis_block);

    // Create a new block
    let block2 = Block::new("Block 2".to_owned(), genesis_block.hash);
    println!("{:?}", block2);

    // Create another block
    let block3 = Block::new("Block 3".to_owned(), block2.hash);
    println!("{:?}", block3);
}
