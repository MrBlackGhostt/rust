use ::chrono::Local;
use ::sha2::{Digest, Sha256};
use chrono::offset::LocalResult;

#[derive(Debug)]
struct Block {
    hash: String,
    prev_hash: String,
    time_stamp: i64,
    data: String,
}
//TODO What is derive Debug
//TODO WHat the format do
//TODO  what thsi {:?} do
#[derive(Debug)]
struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    fn new() -> BlockChain {
        let timestamp = Local::now().timestamp();
        let hash_data = format!("{}{}{}", "0", "", timestamp);
        let block = Block {
            hash: hash(&hash_data),
            prev_hash: String::from("0"),
            data: String::from("GenesisBlock"),
            time_stamp: timestamp,
        };

        BlockChain {
            blocks: vec![block],
        }
    }

    fn add_block(&mut self, data: &str) {
        let timestamp = Local::now().timestamp();
        let last_hash = self.blocks.last().unwrap().hash.clone();
        let hash_data = format!("{}{}{}", data, timestamp, last_hash);

        let block = Block {
            hash: hash(&hash_data),
            data: String::from(data),
            time_stamp: timestamp,
            prev_hash: last_hash,
        };

        self.blocks.push(block);

        println!("{:?}", self.blocks)
    }
}

fn main() {
    let mut blockchain = BlockChain::new();
    println!("Genesis block created\n");

    blockchain.add_block("Alice sends 10 coins to Bob");
    blockchain.add_block("Bob sends 5 coins to Charlie");
    blockchain.add_block("Charlie sends 2 coins to Alice");

    println!("\n📚 Full Blockchain ({} blocks):", blockchain.blocks.len());
    let mut blockchain = BlockChain::new();
    println!("Genesis block created\n");

    blockchain.add_block("Alice sends 10 coins to Bob");
    blockchain.add_block("Bob sends 5 coins to Charlie");
    blockchain.add_block("Charlie sends 2 coins to Alice");

    println!("\n📚 Full Blockchain ({} blocks):", blockchain.blocks.len());
    println!("{:#?}", blockchain);
    println!("{:#?}", blockchain);
}
fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();
    format!("{:x}", result)
}
