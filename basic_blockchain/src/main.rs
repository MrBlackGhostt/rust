use std::fmt::format;

use ::chrono::Local;
use ::sha2::{Digest, Sha256};
use chrono::DateTime;

#[derive(Debug)]
struct BlockChain {
    hash: String,
    prev_hash: String,
    time_stamp: DateTime<Local>,
    data: String,
}
fn main() {
    let time = Local::now();
    println!("Hello, world! time is {}", time);
    let data = "Hello Web3";
    let hash_value = hash(data);
    let prev_hash = "0";
    print!("{}", hash_value);
    print!("{:?}", block("block_no", &data, prev_hash));
}
fn hash(data: &str) -> String {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();
    format!("{:x}", result)
}

fn block(block_no: &str, data: &str, prev_hash: &str) -> BlockChain {
    let time_stamp = Local::now().timestamp();
    let hash_data = format!("{}{}{}{}", data, block_no, prev_hash, time_stamp);
    let block_unit = BlockChain {
        hash: hash(&hash_data),
        time_stamp: Local::now(),
        prev_hash: String::from(prev_hash),
        data: data.to_string(),
    };
    block_unit
}
