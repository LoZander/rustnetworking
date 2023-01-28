//! RSA signature implementation.
//! 
//! This crate features RSA signing and verification.
//! 
//! # Security
//! Because signing and verifying against messages directly allows for forgery attacks,
//! [`sign`] and [`verify`] both work with hashing digests of the messages. This means
//! that signing message `m` actually means signing a hash of it `h(m)`. When verifying
//! we verify `h(m)` against the sign, not `m` directly.
//! 
//! If we did sign and verify on messages directly, an adversary could just pick a random
//! signature `s` and create a message from it. They could then send this message and the signature.
//! This message would be gibberish, but have a valid signature, so it would be a forgery. 
//! Hashing means that reverse engineering `m` would be very difficult if not practically impossible,
//! so the adversary would not be able to send `m` along with `s`, so their plan is foiled.
//! 
//! This implementations uses Sha256[^note] for hashing.s
//! 
//! [^note]: https://en.wikipedia.org/wiki/SHA-2

use sha2::{Sha256, Digest};
use super::{confidentiality::{Message, decrypt, encrypt}, SecretKey, PublicKey};

pub type Signature = Vec<u8>;

/// [`Verification`] represents the result of verifying a signature.
/// 
/// - [`Accept`] represents a successful verification
/// - [`Reject`] represents an unsuccessful verification
pub enum Verification {
    Accept,
    Reject,
}

/// [`sign`] creates an RSA [`Signature`] based on a message and a [`SecretKey`].
/// For RSA signatures, signing is requivalent to decrypting.
/// 
/// # Security
/// To prevent forgery attacks, [`sign`] first hashes a message and then signs the hash.
/// When verfying the signature, the digest of a hashing on the message is compared to the signature.
/// 
/// # Errors
/// Signing can possible fail and so [`sign`] returns a result.
pub fn sign<T: Into<Message>>(message: T, sk: SecretKey) -> Result<Signature,String> {
    let digest: Message = hash(message);
    decrypt(digest, sk)
}

/// [`verify`] verifies a signature against a message and [`PublicKey`].
/// [`verify`] returns a [`Verification`] value which represents whether the 
/// verification accepted or rejected.
/// 
/// # Security
/// To prevent forgery attacks, [`verify`] assumes the signing is done on a hash of the message
/// and so it verifies the signature against not the message, but a hashing of it.
pub fn verify<T: Into<Message>>(message: T, signature: Signature, pk: PublicKey) -> Verification {
    let unsign: Message = encrypt(signature, &pk);
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