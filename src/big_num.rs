use std::{ops::{Add, Sub, Mul, Div, Rem}, fmt::Display};

extern crate num_primes;
use num_primes::{Generator};
extern crate num;
use num::{bigint, FromPrimitive, Integer};
use serde::{Serialize, Deserialize};

pub type Sign = num::bigint::Sign;

#[derive(PartialEq, PartialOrd)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct BigUint {
    inner: num_primes::BigUint
}

impl Serialize for BigUint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let bytes: Vec<u8> = self.clone().into();
        bytes.serialize(serializer)
    }
}

impl <'de>Deserialize<'de> for BigUint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(Vec::<u8>::deserialize(deserializer)?.into())
    }
}

pub enum Digit {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}

impl BigUint {
    pub fn new(digits: Vec<u32>) -> Self {
        BigUint{inner: num_primes::BigUint::new(digits)}
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

    pub fn bit_size(&self) -> u32 {
        self.inner.bits() as u32
    }
}

impl From<Vec<u8>> for BigUint {
    fn from(value: Vec<u8>) -> Self {
        BigUint::from_bytes_be(&value[..])
    }
}

impl From<Digit> for BigUint {
    fn from(value: Digit) -> Self {
        match value {
            Digit::_0 => BigUint::new(vec![0]),
            Digit::_1 => BigUint::new(vec![1]),
            Digit::_2 => BigUint::new(vec![2]),
            Digit::_3 => BigUint::new(vec![3]),
            Digit::_4 => BigUint::new(vec![4]),
            Digit::_5 => BigUint::new(vec![5]),
            Digit::_6 => BigUint::new(vec![6]),
            Digit::_7 => BigUint::new(vec![7]),
            Digit::_8 => BigUint::new(vec![8]),
            Digit::_9 => BigUint::new(vec![9]),
        }
    }
}

impl From<BigUint> for Vec<u8> {
    fn from(value: BigUint) -> Self {
        value.to_bytes_be()
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

impl From<Digit> for BigInt {
    fn from(value: Digit) -> Self {
        match value {
            Digit::_0 => BigInt::new(Sign::Plus,vec![0]),
            Digit::_1 => BigInt::new(Sign::Plus,vec![1]),
            Digit::_2 => BigInt::new(Sign::Plus,vec![2]),
            Digit::_3 => BigInt::new(Sign::Plus,vec![3]),
            Digit::_4 => BigInt::new(Sign::Plus,vec![4]),
            Digit::_5 => BigInt::new(Sign::Plus,vec![5]),
            Digit::_6 => BigInt::new(Sign::Plus,vec![6]),
            Digit::_7 => BigInt::new(Sign::Plus,vec![7]),
            Digit::_8 => BigInt::new(Sign::Plus,vec![8]),
            Digit::_9 => BigInt::new(Sign::Plus,vec![9]),
        }
    }
}

pub fn new_prime(n: usize) -> BigUint {
    BigUint{inner: Generator::new_prime(n)}
}