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

use crate::{big_num::{BigUint}, modular};

use super::{PublicKey, SecretKey, E};

pub type Message = Vec<u8>;
pub type Plaintext = Message;
pub type Ciphertext = Message;


/// [`encrypt`] encrypts a [`Plaintext`] message `m` into a [`Ciphertext`] message `c` using a given [`PublicKey`] pk.
/// The underlying algorithm is `c = m^e mod n`, where `e = 3` is constant and `n` is given by `pk`.
/// 
/// # Examples
/// ```rust
/// use rustnetworking::rsa::{confidentiality::{Plaintext,Ciphertext,encrypt,decrypt},keygen};
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
/// use rustnetworking::rsa::{confidentiality::{Plaintext,Ciphertext,encrypt,decrypt},keygen};
///
/// # fn main() -> Result<(),String> {
/// let (pk,sk) = keygen(2048)?;
/// # let m: Plaintext = "Very secret message ;p".into();
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
    let modulus: BigUint = (p.clone() - big_one.clone())? * (q.clone() - big_one)?;

    let d = modular::inverse(BigUint::from_i32(E).unwrap(), modulus)?;
    Ok(d)
}