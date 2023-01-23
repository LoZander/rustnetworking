use std::{ops::{Add, Sub, Mul, Div, Rem}, fmt::Display};

use num_primes::{self, Generator};
use num::{bigint, FromPrimitive, Integer};

pub type Sign = num::bigint::Sign;

#[derive(PartialEq, PartialOrd)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct BigUint {
    inner: num_primes::BigUint
}

impl BigUint {
    pub fn new(digits: Vec<u32>) -> Self {
        BigUint{inner: num_primes::BigUint::new(digits)}
    }

    pub fn from_i32(value: i32) -> Result<Self,String> {
        let inner = num_primes::BigUint::from_i32(value).ok_or(format!("{value} could not be converted to BigInt"))?;
        Ok(BigUint{inner})
    }

    pub fn modpow(&self, exponent: &Self, modulus: &Self) -> Self {
        BigUint { inner: self.inner.modpow(&exponent.inner, &modulus.inner) }
    }

    pub fn from_bytes_be(bytes: &[u8]) -> BigUint {
        BigUint{inner: num_primes::BigUint::from_bytes_be(bytes)}
    }

    pub fn prime(&self) -> bool {
        num_primes::Verification::is_prime(&self.inner)
    }

    pub fn gcd(&self, other: &Self) -> Self {
        BigUint{inner: self.inner.gcd(&other.inner)}
    }

    pub fn co_prime(&self, other: &Self) -> bool {
        self.gcd(other) == BigUint::new(vec![1])
    }

    pub fn to_bytes_be(&self) -> Vec<u8> {
        self.inner.to_bytes_be()
    }
}

impl Display for BigUint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Rem for BigUint {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        BigUint{inner: self.inner % rhs.inner}
    }
}

impl Add for BigUint {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        BigUint{inner: self.inner + rhs.inner}
    }
}

impl Sub for BigUint {
    type Output = Result<Self,String>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.inner < rhs.inner {
            return Err(format!("{self} is smaller than {rhs}"))
        }
        Ok(BigUint{inner: self.inner - rhs.inner})
    }
}

impl Mul for BigUint {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        BigUint{inner: self.inner * rhs.inner}
    }
}

impl Div for BigUint {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        BigUint{inner: self.inner / rhs.inner}
    }
}


#[derive(PartialEq, PartialOrd)]
#[derive(Clone)]
#[derive(Debug)]
pub struct BigInt {
    inner: bigint::BigInt
}

impl BigInt {
    pub fn new(sign: Sign, digits: Vec<u32>) -> Self {
        BigInt{inner: bigint::BigInt::new(sign, digits)}
    }

    pub fn from_i32(value: i32) -> Result<Self,String> {
        let inner = bigint::BigInt::from_i32(value).ok_or(format!("{value} could not be converted to BigInt"))?;
        Ok(BigInt{inner})
    }

    pub fn gcd(&self, other: &Self) -> Self {
        BigInt{inner: self.inner.gcd(&other.inner)}
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Add for BigInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        BigInt{inner: self.inner + rhs.inner}
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        BigInt{inner: self.inner - rhs.inner}
    }
}

impl Mul for BigInt {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        BigInt{inner: self.inner * rhs.inner}
    }
}

impl Div for BigInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        BigInt{inner: self.inner / rhs.inner}
    }
}

impl From<BigUint> for BigInt {
    fn from(item: BigUint) -> Self {
        BigInt{inner: bigint::BigInt::from_bytes_be(bigint::Sign::Plus, &item.inner.to_bytes_be())}
    }
}

impl From<BigInt> for BigUint {
    fn from(item: BigInt) -> Self {
        BigUint{inner: num_primes::BigUint::from_bytes_be(&item.inner.to_bytes_be().1)}
    }
}

pub fn new_prime(n: usize) -> BigUint {
    BigUint{inner: Generator::new_prime(n)}
}