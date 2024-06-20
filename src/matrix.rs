use std::{
    fmt::Display,
    iter::{repeat, repeat_with},
    ops::{Add, Index, IndexMut, Mul},
};

use rand::distributions::{Distribution, Standard};

use crate::common::Zero;

#[derive(Debug, Clone)]
pub struct Matrix<const NROW: usize, const NCOL: usize, ELEMENT> {
    elements: Vec<ELEMENT>,
}

impl<const NROW: usize, const NCOL: usize, ELEMENT: Zero + Clone> Matrix<NROW, NCOL, ELEMENT> {
    fn size() -> usize {
        NROW * NCOL
    }
    pub fn zero() -> Self {
        Self::new(
            &repeat(ELEMENT::zero())
                .take(Self::size())
                .collect::<Vec<_>>(),
        )
    }

    pub fn new(elements: &[impl Into<ELEMENT> + Clone]) -> Self {
        assert_eq!(elements.len(), Self::size());
        Self {
            elements: elements.iter().map(|x| (*x).clone().into()).collect(),
        }
    }

    /// Transpose
    pub fn t(&self) -> Matrix<NCOL, NROW, ELEMENT> {
        let mut elements = vec![];
        for new_row in 0..NCOL {
            for new_col in 0..NROW {
                elements.push(self[(new_col, new_row)].clone())
            }
        }
        Matrix { elements }
    }
}
impl<const NROW: usize, const NCOL: usize, ELEMENT: Clone + Display> Display
    for Matrix<NROW, NCOL, ELEMENT>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        writeln!(f, "[")?;
        for row in self.elements.chunks_exact(NCOL) {
            write!(f, "  [")?;
            for elem in row.iter() {
                write!(f, " {elem} ")?;
            }
            writeln!(f, "]")?;
        }

        writeln!(f, "]")
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
                .map(|(self_elem, rhs_elem)| self_elem.clone() + rhs_elem.clone())
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
        &self.elements[row * NCOL + col]
    }
}

impl<const NROW: usize, const NCOL: usize, ELEMENT> IndexMut<(usize, usize)>
    for Matrix<NROW, NCOL, ELEMENT>
{
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.elements[row * NCOL + col]
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
            for j in 0..NCOL {
                let mut sum = ELEMENT::zero();
                for k in 0..M {
                    sum = sum + self[(i, k)].clone() * rhs[(k, j)].clone();
                }
                result[(i, j)] = sum;
            }
        }
        result
    }
}

impl<const NROW: usize, const NCOL: usize, ELEMENT> Distribution<Matrix<NROW, NCOL, ELEMENT>>
    for Standard
where
    Standard: Distribution<ELEMENT>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Matrix<NROW, NCOL, ELEMENT> {
        Matrix {
            elements: repeat_with(|| rng.gen())
                .take(NROW * NCOL)
                .collect::<Vec<_>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ff::PrimeField;

    use super::*;

    #[test]
    fn test_matrix() {
        type M = Matrix<2, 2, PrimeField<11>>;
        let matrix_a: M = Matrix::new(&[3, 4, 5, 6]);
        let matrix_b: M = Matrix::new(&[7, 8, 9, 10]);
        let matrix_b_t: M = Matrix::new(&[7, 9, 8, 10]);
        let a_plus_b: M = Matrix::new(&[10, 1, 3, 5]);
        let a_mul_b = Matrix::new(&[2, 9, 1, 1]);
        println!("Matrix a {matrix_a}");
        assert_eq!(matrix_a.clone() + matrix_b.clone(), a_plus_b);
        assert_eq!(matrix_a * matrix_b.clone(), a_mul_b);
        assert_eq!(matrix_b.t(), matrix_b_t);
    }
}
