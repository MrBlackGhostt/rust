use std::collections::HashMap;

struct Mint {
    decimal: u8,
    total_supply: u64,
    mint_authority: String,
    freeze_authority: Option<String>,
}

struct TokenAccount {
    account_id: String, // hex string
    owner_id: String,
    amount: u64,
    mint_id: String,
    frozen: bool,
}
struct Owner {
    owner_id: String,
    mint_id: String,
    public_key: String,
    private_key: String,
}

struct Ledger {
    mint: HashMap<String, Mint>,
    account: HashMap<String, TokenAccount>,
}

fn main() {
    println!("Hello, world!");
}
