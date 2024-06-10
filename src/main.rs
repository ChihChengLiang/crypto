use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut, Mul},
};

trait Zero {
    fn zero() -> Self;
}

#[derive(Debug, Clone)]
struct PrimeField<const P: u64> {
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

fn fp11(n: u64) -> PrimeField<11> {
    PrimeField::new(n)
}

#[derive(Debug, Clone)]
struct Matrix<const NROW: usize, const NCOL: usize, ELEMENT> {
    elements: Vec<Vec<ELEMENT>>,
}

impl<const NROW: usize, const NCOL: usize, ELEMENT: Zero + Clone> Matrix<NROW, NCOL, ELEMENT> {
    fn zero() -> Self {
        Self {
            elements: (0..NROW)
                .map(|_| (0..NCOL).map(|_| ELEMENT::zero()).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

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

impl<const NROW: usize, const NCOL: usize, ELEMENT: Add<Output = ELEMENT> + Clone> Add
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

impl<const NROW: usize, const NCOL: usize, ELEMENT> Index<(usize, usize)>
    for Matrix<NROW, NCOL, ELEMENT>
{
    type Output = ELEMENT;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        self.elements
            .get(row)
            .expect("row exists")
            .get(col)
            .expect("elem exists")
    }
}

impl<const NROW: usize, const NCOL: usize, ELEMENT> IndexMut<(usize, usize)>
    for Matrix<NROW, NCOL, ELEMENT>
{
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.elements[row][col]
    }
}

impl<
        const NROW: usize,
        const M: usize,
        const NCOL: usize,
        ELEMENT: Mul<Output = ELEMENT> + Add<Output = ELEMENT> + Zero + Clone,
    > Mul<Matrix<M, NCOL, ELEMENT>> for Matrix<NROW, M, ELEMENT>
{
    type Output = Matrix<NROW, NCOL, ELEMENT>;

    fn mul(self, rhs: Matrix<M, NCOL, ELEMENT>) -> Self::Output {
        let mut result = Matrix::zero();
        for i in 0..NROW {
            for j in 0..M {
                let mut sum = ELEMENT::zero();
                for k in 0..NCOL {
                    sum = sum + self[(i, k)].clone() * rhs[(k, j)].clone();
                }
                result[(i, j)] = sum;
            }
        }
        result
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
        let matrix_a: Matrix<2, 2, _> = Matrix::new(&[&[fp11(3), fp11(4)], &[fp11(5), fp11(6)]]);
        let matrix_b: Matrix<2, 2, _> = Matrix::new(&[&[fp11(7), fp11(8)], &[fp11(9), fp11(10)]]);
        let a_plus_b: Matrix<2, 2, _> = Matrix::new(&[&[fp11(10), fp11(1)], &[fp11(3), fp11(5)]]);
        let a_mul_b = Matrix::new(&[&[fp11(2), fp11(9)], &[fp11(1), fp11(1)]]);
        println!("Matrix a {matrix_a}");
        assert_eq!(matrix_a.clone() + matrix_b.clone(), a_plus_b);
        assert_eq!(matrix_a * matrix_b, a_mul_b)
    }
}

fn main() {
    println!("Hello, world!");
}
