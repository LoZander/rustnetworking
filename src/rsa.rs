use rand;
use num_primes::{Generator, BigUint};

type Message = Vec<u8>;
type Cipher = Vec<u8>;
const e: u32 = 3;
pub struct PublicKey {
    n: BigUint
}

pub struct SecretKey {
    p: BigUint,
    q: BigUint
}

pub enum Key {
    PublicKey,
    SecretKey
}

pub fn generate(bit_size: u32) -> Result<(PublicKey,SecretKey),String> {
    let random_difference = rand::random::<f32>() * (bit_size as f32) / 10. as f32;
    let p_size = (bit_size as f32 / 2. + random_difference).floor() as usize;
    let q_size = (bit_size as f32 / 2. - random_difference).ceil() as usize;

    fn f(p_size: usize,q_size: usize) -> Result<(BigUint,BigUint), String> {
        let p_candidate = Generator::new_prime(p_size);
        let q_candidate = Generator::new_prime(q_size);
        
        let mut vec_of_bound = vec![0; p_size + q_size];
        vec_of_bound.insert(vec_of_bound.len() - 1,1);
        let bound = BigUint::new(vec_of_bound);

        if p_candidate.clone() * q_candidate.clone() > bound {
            return f(p_size, q_size)
        }

        return Ok((p_candidate,q_candidate))
    }

    let (p,q) = f(p_size,q_size)?;
    let n = p.clone() * q.clone();

    let public_key = PublicKey{n};
    let secret_key = SecretKey{p,q};
    
    return Ok((public_key, secret_key))
}

pub fn encrypt(m: Message, pk: PublicKey) -> Cipher {
    let m_number = number_from_bytes(m);
    let cipher = m_number.modpow(&BigUint::from(e), &pk.n);
    return bytes_from_number(cipher)
}

fn number_from_bytes(m: Message) -> BigUint {
    return BigUint::from_bytes_be(&m)
}

fn bytes_from_number(c: BigUint) -> Cipher {
    return c.to_bytes_be()
}

pub fn decrypt(c: Cipher, sk: SecretKey) -> Message {
    let c_number = number_from_bytes(c);
    let d = create_d(&sk.p,&sk.q);

    let message = c_number.modpow(&d, &(sk.p * sk.q));
    return bytes_from_number(message);
}

fn create_d(p: &BigUint,q: &BigUint) -> BigUint {
    let big_one = BigUint::new(vec![1]);
    let modulus = (p - &big_one) * (q - &big_one);

    let d = BigUint::from(e) % modulus; //Alex; 22/01/2023; TODO: do modular inverse d = e^(-1) mod (p - 1)(q - 1) here!
    return d;
}