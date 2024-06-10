use ff::PrimeField;
use matrix::Matrix;

mod common;
mod ff;
mod matrix;

struct DataPoint {
    xs: Matrix<1, 4, PrimeField<11>>,
    y: PrimeField<11>,
}

impl DataPoint {
    fn new(xs: &[u64; 4], y: u64) -> Self {
        Self {
            xs: Matrix::new(xs),
            y: y.into(),
        }
    }
}

fn main() {
    let blue_set = [
        ((1, 0, 1, 7), 2),
        ((5, 8, 4, 10), 9),
        ((7, 7, 8, 5), 3),
        ((5, 1, 10, 6), 3),
        ((8, 0, 2, 4), 1),
        ((9, 3, 0, 6), 9),
        ((0, 6, 1, 6), 9),
        ((0, 4, 9, 7), 5),
        ((10, 7, 4, 10), 10),
        ((5, 5, 10, 6), 8),
        ((10, 7, 3, 1), 9),
        ((0, 2, 5, 5), 6),
        ((9, 10, 2, 1), 2),
        ((3, 7, 2, 1), 5),
        ((2, 3, 4, 5), 3),
        ((2, 1, 6, 9), 3),
    ];
    let red_set = [
        ((5, 4, 5, 2), 2),
        ((7, 7, 7, 8), 5),
        ((6, 8, 2, 2), 0),
        ((10, 4, 4, 3), 1),
        ((1, 10, 8, 6), 6),
        ((2, 7, 7, 4), 4),
        ((8, 6, 6, 9), 1),
        ((10, 6, 1, 6), 9),
        ((3, 1, 10, 9), 7),
        ((2, 4, 10, 3), 7),
        ((10, 4, 6, 4), 2),
        ((8, 5, 7, 2), 2),
        ((4, 7, 0, 0), 8),
        ((0, 3, 0, 0), 0),
        ((8, 3, 2, 7), 8),
        ((4, 6, 6, 3), 2),
    ];
}
