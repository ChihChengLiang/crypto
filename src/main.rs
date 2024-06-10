use std::{
    fmt::{write, Display},
    ops::{Add, Mul},
};

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
impl Display for PrimeField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fp{}", self.n)
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

#[derive(Debug)]
struct Matrix<const NROW: usize, const NCOL: usize, ELEMENT> {
    elements: Vec<Vec<ELEMENT>>,
}

impl<const NROW: usize, const NCOL: usize, ELEMENT: Clone> Matrix<NROW, NCOL, ELEMENT> {
    fn new(elements: &[&[ELEMENT]]) -> Self {
        Self {
            elements: elements
                .iter()
                .map(|v| v.to_vec())
                .collect::<Vec<_>>()
                .to_vec(),
        }
    }
}
impl<const NROW: usize, const NCOL: usize, ELEMENT: Clone + Display> Display
    for Matrix<NROW, NCOL, ELEMENT>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[\n")?;
        for row in self.elements.iter() {
            write!(f, "  [")?;
            for elem in row.iter() {
                write!(f, "{elem}, ")?;
            }
            write!(f, "]\n")?;
        }

        write!(f, "]\n")
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
    #[test]
    fn test_matrix() {
        let m: Matrix<2, 2, PrimeField> = Matrix::new(&[
            &[PrimeField::new(3, 11), PrimeField::new(4, 11)],
            &[PrimeField::new(5, 11), PrimeField::new(6, 11)],
        ]);
        println!("Matrix {m}")
    }
}

fn main() {
    println!("Hello, world!");
}
