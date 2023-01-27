//! RSA encryption implementation.
//! 
//! This crate features RSA encryption and decryption.
//! 
//! # RSA
//! RSA is a public-key, computational cryptography scheme.
//! Computational refers to the fact that the system is only secure under
//! the assumption that a given adversary has bounded time and resources.
//! 
//! "Public-key", or alternaively "asymetrical", means that this scheme has a different key
//! for the reader, secret-key, and the sender, public-key. For a keypair, encryption is done with the public-key,
//! which anyone can do, but only the owner of the secret-key can decrypt the message with this key.
//! 
//! # Security
//! RSA has various security issues when used alone. It's therefore recommended to use OAEP[^note].
//! 
//! [^note]: `https://en.wikipedia.org/wiki/Optimal_asymmetric_encryption_padding`

use crate::{big_num::{BigUint, new_prime}, modular};

pub type Message = Vec<u8>;
pub type Plaintext = Message;
pub type Ciphertext = Message;
const E: i32 = 3;
pub struct PublicKey {
    n: BigUint
}

impl PublicKey {
    pub fn bit_size(&self) -> u32 {
        self.n.bit_size()
    }
}

pub struct SecretKey {
    p: BigUint,
    q: BigUint
}

pub type KeyPair = (PublicKey, SecretKey);
pub enum Key {
    PublicKey,
    SecretKey
}


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
/// use rustnetworking::rsa::{Plaintext,Ciphertext,encrypt,decrypt,keygen};
/// #
/// # fn main() -> Result<(),String> {
/// let (pk,sk) = keygen(2048)?;
/// 
/// let m: Plaintext = "Very secret message ;p".as_bytes().into();
/// let c: Ciphertext = encrypt(m, pk).into();
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

/// [`encrypt`] encrypts a [`Plaintext`] message `m` into a [`Ciphertext`] message `c` using a given [`PublicKey`] pk.
/// The underlying algorithm is `c = m^e mod n`, where `e = 3` is constant and `n` is given by `pk`.
/// 
/// # Examples
/// ```rust
/// use rustnetworking::rsa::{Plaintext,Ciphertext,encrypt,decrypt,keygen};
/// #
/// # fn main() -> Result<(),String> {
/// let (pk,sk) = keygen(2048)?;
/// 
/// let m: Plaintext = "Very secret message ;p".as_bytes().into();
/// let c: Ciphertext = encrypt(m, pk).into();
/// # Ok(())
/// # }
/// ```
pub fn encrypt<T: Into<Plaintext>>(plaintext: T, pk: PublicKey) -> Ciphertext {
    let plaintext_as_number: BigUint = plaintext.into().into();
    let cipher = plaintext_as_number.modpow(&BigUint::from_i32(E).unwrap(), &pk.n);
    cipher.into()
}


/// [`decrypt`] decrypts a [`Ciphertext`] message `c` into its original [`Plaintext`] form `m`
/// using a [`SecretKey`] sk.
/// The underlying algorithm is `m = c^d mod n`, where `d` is the modular inverse 
/// 
/// `d = e^(-1) mod (p - 1)(q - 1)`. 
/// 
/// `p,q` are given by `pk`.
/// 
/// [`decrypt`] returns a [`Result<Plaintext,String>`] because the 
/// function might fail if given a wrong key [`SecretKey`].
/// 
/// # Examples
/// ```rust
/// use rustnetworking::rsa::{Plaintext,Ciphertext,encrypt,decrypt,keygen};
///
/// # fn main() -> Result<(),String> {
/// let (pk,sk) = keygen(2048)?;
/// # let m: Plaintext = "Very secret message ;p".as_bytes().into();
/// # let c: Ciphertext = encrypt(m, pk).into();
/// // ...
/// let decrypted = decrypt(c, sk)?;
/// # Ok(())
/// # }
/// ```
/// 
/// # Errors
/// [`decrypt`] gives an error when given a bad or wrong [`SecretKey`],
pub fn decrypt<T: Into<Ciphertext>>(ciphertext: T, sk: SecretKey) -> Result<Plaintext,String> {
    let ciphertext_number: BigUint = ciphertext.into().into();
    let d = create_d(&sk.p,&sk.q).map_err(|_| "bad key")?;

    let message = ciphertext_number.modpow(&d, &(sk.p * sk.q));
    Ok(message.into())
}

fn create_d(p: &BigUint,q: &BigUint) -> Result<BigUint,String> {
    let big_one = BigUint::from_i32(1).unwrap();
    let modulus: BigUint = (p.clone() - big_one.clone().into())? * (q.clone() - big_one.into())?;

    let d = modular::inverse(BigUint::from_i32(E).unwrap(), modulus)?;
    Ok(d)
}