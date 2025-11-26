use clap::{Parser, Subcommand};
use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use hex;
use rand::rngs::OsRng;

#[derive(Parser)]
#[command(name = "Crypto_wallet")]
#[command(about = "A simle crypt wallet")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create,
    Sign { message: String },
    Verify { signature: String, message: String },
}

#[derive(Debug)]
struct crypto_wallet {
    keypair: Keypair,
}

impl crypto_wallet {
    fn new() -> crypto_wallet {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        crypto_wallet { keypair: keypair }
    }
    fn show_pub_key(&self) -> String {
        hex::encode(&self.keypair.public)
    }
    fn show_pri_key(&self) -> String {
        hex::encode(&self.keypair.secret)
    }

    fn sign(&self, msg: &str) -> String {
        let msg_bytes = msg.as_bytes();

        let signature = self.keypair.sign(msg_bytes); // is it give the signature into the
        hex::encode(signature)
    }

    fn verify_sign(&self, signature: &str, message: &str) -> bool {
        let sign_bytes = match hex::decode(signature) {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        // Convert bytes to Signature struct
        let signature = match Signature::from_bytes(&sign_bytes) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        self.keypair
            .public
            .verify(message.as_bytes(), &signature)
            .is_ok()
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create => {
            let wallet = crypto_wallet::new();

            println!("🔑 Wallet Created!");
            println!("📍 Public Key:  {}", wallet.show_pub_key());
            println!("🔒 Private Key: {}\n", wallet.show_pri_key());
        }

        Commands::Sign { message } => {
            let wallet = crypto_wallet::new();
            let signature = wallet.sign(&message);

            println!("📝 Signature: {:?}\n", signature);
        }
        Commands::Verify { signature, message } => {
            let wallet = crypto_wallet::new();
            let signature = wallet.sign(&message);
            println!("the wallet is {}", wallet.verify_sign(&signature, &message));
        }
    }
}
