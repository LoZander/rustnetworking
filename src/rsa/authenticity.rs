use sha2::{Sha256, Digest};
use super::{confidentiality::{Message, decrypt, encrypt}, SecretKey, PublicKey};

pub type Signature = Vec<u8>;
pub enum Verification {
    Accept,
    Reject,
}

pub fn sign<T: Into<Message>>(message: T, sk: SecretKey) -> Result<Signature,String> {
    let digest: Message = hash(message);
    decrypt(digest, sk)
}

pub fn verify<T: Into<Message>>(message: T, signature: Signature, pk: PublicKey) -> Verification {
    let unsign: Message = encrypt(signature, pk);
    if hash(message) == unsign {
        Verification::Accept
    } else {
        Verification::Reject
    }
}

fn hash<T: Into<Message>>(message: T) -> Message {
    let mut hasher = Sha256::new();
    hasher.update(message.into());
    hasher.finalize().to_vec()
}