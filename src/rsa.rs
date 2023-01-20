use rand;
use num_primes::{Generator, BigUint};

type Message = String;
type Cipher = String;

pub struct PublicKey {
    e: BigUint
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

    let public_key = PublicKey{e: BigUint::new(vec![3])};
    let secret_key = SecretKey{p,q};
    
    return Ok((public_key, secret_key))
}

pub fn encrypt(m: Message) -> Cipher {
    todo!()
}

pub fn decrypt(c: Cipher) -> Message {
    todo!()
}