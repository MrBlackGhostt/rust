use ed25519_dalek::{Keypair, Signature, Signer, Verifier};
use hex;
use rand::rngs::OsRng;

#[derive(Debug)]
struct Crypto_wallet {
    keypair: Keypair,
}

impl Crypto_wallet {
    fn new() -> Crypto_wallet {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);
        Crypto_wallet { keypair: keypair }
    }
    fn show_pub_key(&self) -> String {
        hex::encode(&self.keypair.public)
    }
    fn show_pri_key(&self) -> String {
        hex::encode(&self.keypair.secret)
    }

    fn sign(&self, msg: &str) -> Signature {
        let msg_bytes = msg.as_bytes();

        let signature = self.keypair.sign(msg_bytes); // is it give the signature into the
        signature
    }

    fn verify_sign(
        &self,
        signature: &Signature,
        message: &str,
    ) -> Result<(), ed25519_dalek::ed25519::Error> {
        let v = self.keypair.public.verify(message.as_bytes(), signature);
        v
    }
}

fn main() {
    let wallet = Crypto_wallet::new();

    println!("🔑 Wallet Created!");
    println!("📍 Public Key:  {}", wallet.show_pub_key());
    println!("🔒 Private Key: {}\n", wallet.show_pri_key());

    let message = "Hello world";
    let signature = wallet.sign(message);

    println!("✍️  Message: {}", message);
    println!("📝 Signature: {:?}\n", signature);

    // Verify
    match wallet.verify_sign(&signature, message) {
        Ok(_) => println!("✅ Signature is VALID!"),
        Err(_) => println!("❌ Signature is INVALID!"),
    }

    // Test with wrong message
    println!("\n🧪 Testing with wrong message...");
    match wallet.verify_sign(&signature, "Wrong message") {
        Ok(_) => println!("✅ Valid (This shouldn't happen!)"),
        Err(_) => println!("❌ Invalid (Expected!)"),
    }
}
