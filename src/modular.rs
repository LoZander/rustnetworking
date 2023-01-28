//! A collection of modular arithmetic functions
//! 
//! For more about modular arithmetic see
//! `https://en.wikipedia.org/wiki/Modular_arithmetic`

use crate::big_num::{BigInt,BigUint, Digit};

/// [`inverse`] calculates the modular inverse `x^(-1) (mod modulus)`.
/// 
/// The modular inverse `a^(-1)` is and integer such that `aa^(-1) = 1 (mod n)`.
/// Remark that (mod n) is distinct from the normal binary % operation[^note].
/// 
/// [^note]: See `https://en.wikipedia.org/wiki/Modular_arithmetic`
/// 
/// This implementation is based on the algorithm found at 
/// `https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Computing_multiplicative_inverses_in_modular_structures`
/// 
/// # Errors
/// An error is returned if the input is invertable.
/// 
pub fn inverse(x: BigUint, modulus: BigUint) -> Result<BigUint,String> {
    struct FData {
        t: BigInt,
        new_t: BigInt,
        r: BigInt,
        new_r: BigInt,
    }

    fn f(data: FData) -> FData {
        if data.new_r == Digit::_0.into() {
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
        t: Digit::_0.into(),
        new_t: Digit::_1.into(),
        r: modulus.clone().into(),
        new_r: x.into(),
    });

    if data.r > Digit::_1.into() {
        return Err("x is not invertable".into())
    }

    if data.t < Digit::_0.into() {
        return Ok((data.t + modulus.into()).into())
    }

    Ok(data.t.into())
}