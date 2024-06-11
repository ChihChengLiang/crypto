use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use crate::common::Zero;

#[derive(Debug, Clone)]
pub struct PrimeField<const P: u64> {
    n: u64,
}

impl<const P: u64> PrimeField<P> {
    fn new(n: u64) -> Self {
        Self { n: n % P }
    }

    fn neg(&self) -> Self {
        Self { n: P - self.n }
    }

    /// Inverse via Fermat's little theorem
    fn invert(&self) -> Self {
        assert_ne!(self.n, 0, "Can't invert zero");
        Self {
            n: self.n.pow((P - 2).try_into().unwrap()) % P,
        }
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.n == 0
    }

    pub fn check_tolerance(&self, tolerance: u64) -> bool {
        self.n <= tolerance
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

impl<const P: u64> Sub for PrimeField<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg()
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

impl<const P: u64> Div for PrimeField<P> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(!rhs.is_zero());
        self * rhs.invert()
    }
}

impl<const P: u64> PartialEq for PrimeField<P> {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n
    }
}

impl<const P: u64> From<u64> for PrimeField<P> {
    fn from(n: u64) -> Self {
        Self::new(n)
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
        for i in 1..11 {
            assert_eq!(fp11(i) / fp11(i), fp11(1), "Identity for {i} unsatisfied")
        }
    }

    #[test]
    fn test_tolerance() {
        let tolerance = 2;
        for i in 0..11 {
            assert_eq!(
                fp11(i).check_tolerance(tolerance),
                [0, 1, 2, 10, 9, 8].contains(&i)
            )
        }
    }
}
