use std::{
    fmt::Display,
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

fn fp11(n: u64) -> PrimeField {
    PrimeField::new(n, 11)
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

impl<const NROW: usize, const NCOL: usize, ELEMENT: Clone + Add<Output = ELEMENT>> Add
    for Matrix<NROW, NCOL, ELEMENT>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            elements: self
                .elements
                .iter()
                .zip(rhs.elements.iter())
                .map(|(self_row, rhs_row)| {
                    let new_row: Vec<ELEMENT> = self_row
                        .iter()
                        .zip(rhs_row.iter())
                        .map(|(self_elem, rhs_elem)| self_elem.clone() + rhs_elem.clone())
                        .collect();
                    new_row
                })
                .collect::<Vec<_>>(),
        }
    }
}

impl<const NROW: usize, const NCOL: usize, ELEMENT: PartialEq> PartialEq
    for Matrix<NROW, NCOL, ELEMENT>
{
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
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
    #[test]
    fn test_matrix() {
        let matrix_a: Matrix<2, 2, PrimeField> =
            Matrix::new(&[&[fp11(3), fp11(4)], &[fp11(5), fp11(6)]]);
        let matrix_b: Matrix<2, 2, PrimeField> =
            Matrix::new(&[&[fp11(7), fp11(8)], &[fp11(9), fp11(10)]]);
        let a_plus_b: Matrix<2, 2, PrimeField> =
            Matrix::new(&[&[fp11(10), fp11(1)], &[fp11(3), fp11(5)]]);
        assert_eq!(matrix_a + matrix_b, a_plus_b)
    }
}

fn main() {
    println!("Hello, world!");
}
