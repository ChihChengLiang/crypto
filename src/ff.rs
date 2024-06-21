use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use rand::distributions::{Distribution, Standard, Uniform};
use rand_distr::{num_traits::Float, Normal, StandardNormal};

use crate::{
    common::{Raisable, Zero},
    utils::round_frac2int,
};

#[derive(Debug, Clone)]
pub struct PrimeField<const P: u64> {
    n: u64,
}

impl<const P: u64> PrimeField<P> {
    pub(crate) fn new(n: u64) -> Self {
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

    pub fn round_2int(&self, q: u64) -> u64 {
        round_frac2int(self.n, q)
    }
    pub fn cast<const Q: u64>(&self) -> PrimeField<Q> {
        PrimeField::<Q> { n: self.n % Q }
    }
}
impl<const P: u64> Display for PrimeField<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.n)
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

impl<const P: u64> Raisable for PrimeField<P> {
    fn pow(&self, power: usize) -> Self {
        Self {
            n: self.n.pow(power.try_into().unwrap()) % P,
        }
    }
}

impl<const P: u64> Distribution<PrimeField<P>> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PrimeField<P> {
        PrimeField::new(Uniform::new(0, P).sample(rng))
    }
}

impl<const P: u64, F: Float> Distribution<PrimeField<P>> for Normal<F>
where
    StandardNormal: Distribution<F>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> PrimeField<P> {
        let value: F = rng.sample(self);
        let value = value.round().to_f32().unwrap() as i32;
        let value = if value < 0 {
            let tmp = (-value) as u64 % P;
            P - tmp
        } else {
            value as u64 % P
        };
        PrimeField::new(value)
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;

    fn fp11(n: u64) -> PrimeField<11> {
        PrimeField::new(n)
    }

    #[test]
    fn test_prime_field() {
        let a = fp11(3);
        let b = fp11(5);
        assert_eq!(a.clone() + b.clone(), fp11(8));
        assert_eq!(a.clone() * b, fp11(4));
        for i in 1..11 {
            assert_eq!(fp11(i) / fp11(i), fp11(1), "Identity for {i} unsatisfied");
        }
        // Test power
        assert_eq!(a.pow(5), fp11(1));
        for i in 0..11 {
            for j in 0..11 {
                assert_eq!(
                    fp11(i).pow(j),
                    iter::repeat(fp11(i)).take(j).fold(fp11(1), |a, b| a * b)
                )
            }
        }
    }

    #[test]
    fn test_tolerance() {
        let tolerance = 2;
        for i in 0..11 {
            assert_eq!(fp11(i).check_tolerance(tolerance), [0, 1, 2,].contains(&i))
        }
    }

    #[test]
    fn test_normal() {
        let rng = ChaCha8Rng::seed_from_u64(123456);
        let xs: Vec<PrimeField<31>> = rng
            .sample_iter(Normal::<f32>::new(0.0, 1.0).unwrap())
            .take(10)
            .collect();
        assert_eq!(
            xs,
            vec![1u64, 30, 0, 0, 0, 1, 29, 1, 0, 1]
                .iter()
                .map(|x| PrimeField::<31>::from(*x))
                .collect::<Vec<_>>()
        )
    }
}
