use crate::big_num::{BigUint, new_prime};

pub mod confidentiality;
pub mod authenticity;

#[derive(Clone)]
pub struct PublicKey {
    n: BigUint
}

impl PublicKey {
    pub fn bit_size(&self) -> u32 {
        self.n.bit_size()
    }
}

#[derive(Clone)]
pub struct SecretKey {
    p: BigUint,
    q: BigUint
}

pub type KeyPair = (PublicKey, SecretKey);
pub enum Key {
    PublicKey,
    SecretKey
}

pub const E: i32 = 3;


/// [`keygen`] generates an RSA [`KeyPair`] with a given `bit_size`.
/// The `bit_size` is the size of `n = p * q`, where `p,q` are large prime numbers.
/// [`keygen`] returns a [`Result<KeyPair,String>`].
/// 
/// The [`KeyPair`] contains two keys
/// - A [`PublicKey`] that can be used in [`encrypt`] to encrypt a message.
/// - A [`SecretKey`] that can be used in [`decrypt`] to decrypt a message.
/// 
/// # Examples
/// ```rust
/// use rustnetworking::rsa::{confidentiality::{Plaintext,Ciphertext,encrypt,decrypt},keygen};
/// #
/// # fn main() -> Result<(),String> {
/// let (pk,sk) = keygen(2048)?;
/// 
/// let m: Plaintext = "Very secret message ;p".as_bytes().into();
/// let c: Ciphertext = encrypt(m, &pk).into();
/// let decrypted = decrypt(c, sk)?;
/// # Ok(())
/// # }
/// ```
/// 
/// # Security
/// For security, it's recommended to use `bit_size` 2048 or larger.
/// 
/// # Errors
/// Keygen handles errors by propagating them. Errors should not occur unless a very small `bit_size` is chosen.
/// 
/// # Panics
/// Keygen should not panic under normal circumstances.
pub fn keygen(bit_size: u32) -> Result<KeyPair,String> {
    let p_size = (bit_size as f32 / 2.).floor() as usize;
    let q_size = (bit_size as f32 / 2.).ceil() as usize;

    fn f(p_size: usize,q_size: usize) -> Result<(BigUint,BigUint), String> {
        let p_candidate = new_prime(p_size);
        let q_candidate = new_prime(q_size);

        if p_candidate == q_candidate {
            return f(p_size,q_size)
        }

        if p_candidate.bit_size() + q_candidate.bit_size() > p_size as u32 + q_size as u32 {
            return f(p_size, q_size)
        }

        let one = BigUint::from_i32(1)?;
        let modulus = (p_candidate.clone() - one.clone())? * (q_candidate.clone() - one)?;
        if !BigUint::from_i32(E)?.co_prime(&modulus) {
            return f(p_size, q_size)
        }

        Ok((p_candidate,q_candidate))
    }

    let (p,q) = f(p_size,q_size)?;

    let n = p.clone() * q.clone();

    let public_key = PublicKey{n};
    let secret_key = SecretKey{p,q};
    Ok((public_key, secret_key))
}