use std::{
    fmt::Display,
    ops::{Add, Mul},
};

use crate::common::Zero;

#[derive(Debug, Clone)]
pub(crate) struct PrimeField<const P: u64> {
    n: u64,
}

impl<const P: u64> PrimeField<P> {
    fn new(n: u64) -> Self {
        Self { n: n % P }
    }
}
impl<const P: u64> Display for PrimeField<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "F{P}{{{}}}", self.n)
    }
}

impl<const P: u64> Zero for PrimeField<P> {
    fn zero() -> Self {
        Self { n: 0 }
    }
}

impl<const P: u64> Add for PrimeField<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            n: (self.n + rhs.n) % P,
        }
    }
}

impl<const P: u64> Mul for PrimeField<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            n: (self.n * rhs.n) % P,
        }
    }
}

impl<const P: u64> PartialEq for PrimeField<P> {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

pub(crate) fn fp11(n: u64) -> PrimeField<11> {
    PrimeField::new(n)
}
mod tests {
    use super::*;

    #[test]
    fn test_prime_field() {
        let a = fp11(3);
        let b = fp11(5);
        assert_eq!(a.clone() + b.clone(), fp11(8));
        assert_eq!(a * b, fp11(4));
    }
}
