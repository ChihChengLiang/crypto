use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
struct PrimeField {
    n: u64,
    p: u64,
}

impl PrimeField {
    fn new(n: u64, p: u64) -> Self {
        Self { n: n % p, p }
    }
}

impl Add for PrimeField {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            n: (self.n + rhs.n) % self.p,
            p: self.p,
        }
    }
}

impl Mul for PrimeField {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            n: (self.n * rhs.n) % self.p,
            p: self.p,
        }
    }
}

impl PartialEq for PrimeField {
    fn eq(&self, other: &Self) -> bool {
        assert_eq!(self.p, other.p);
        self.n == other.n
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_prime_field() {
        let a = PrimeField::new(3, 11);
        let b = PrimeField::new(5, 11);
        assert_eq!(a.clone() + b.clone(), PrimeField::new(8, 11));
        assert_eq!(a * b, PrimeField::new(4, 11));
    }
}

fn main() {
    println!("Hello, world!");
}
