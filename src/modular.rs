//! A collection of modular arithmetic functions
//! 
//! For more about modular arithmetic see
//! `https://en.wikipedia.org/wiki/Modular_arithmetic`

use crate::big_num::{BigInt,BigUint,Sign};

/// [`inverse`] calculates the modular inverse `x^(-1) (mod modulus)`.
/// 
/// The modular inverse `a^(-1)` is and integer such that `aa^(-1) = 1 (mod n).
/// Remark that (mod n) is distinct from the normal binary % operation[^note].
/// 
/// [^note]: See `https://en.wikipedia.org/wiki/Modular_arithmetic`
/// 
/// This implementation is based on the algorithm found at 
/// `https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures`
pub fn inverse(x: BigUint, modulus: BigUint) -> Result<BigUint,String> {
    struct FData {
        t: BigInt,
        new_t: BigInt,
        r: BigInt,
        new_r: BigInt,
    }

    fn f(data: FData) -> FData {
        if data.new_r == BigInt::new(Sign::Plus, vec![0]) {
            return data
        }

        let quotient = data.r.clone() / data.new_r.clone();
        let (t, new_t) = g(data.t, data.new_t, quotient.clone());
        let (r, new_r) = g(data.r, data.new_r, quotient);

        f(FData{
            t,
            new_t,
            r,
            new_r
        })
    }

    fn g(x: BigInt, new_x: BigInt, quotient: BigInt) -> (BigInt, BigInt) {
        (new_x.clone(), x - quotient * new_x)
    }

    let data = f(FData {
        t: BigInt::from_i32(0)?,
        new_t: BigInt::from_i32(1)?,
        r: BigInt::from(modulus.clone()),
        new_r: BigInt::from(x),
    });

    if data.r > BigInt::from_i32(1)? {
        return Err("x is not invertable".to_string())
    }

    if data.t < BigInt::from_i32(0)? {
        return Ok((data.t + modulus.into()).into())
    }

    Ok(data.t.into())
}