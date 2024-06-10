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

    fn try_key(&self, key: &Matrix<4, 1, PrimeField<11>>, tolerance: u64) -> bool {
        let error = self.y.clone() - (self.xs.clone() * key.clone())[(0, 0)].clone();
        error.check_tolerance(tolerance)
    }
}

struct DataSet {
    data: Vec<DataPoint>,
}
type Key = Matrix<4, 1, PrimeField<11>>;
impl DataSet {
    fn new(values: &[((u64, u64, u64, u64), u64)]) -> Self {
        Self {
            data: values
                .iter()
                .map(|((x1, x2, x3, x4), y)| DataPoint::new(&[*x1, *x2, *x3, *x4], *y))
                .collect::<Vec<_>>(),
        }
    }

    fn search(&self) -> (Key, usize) {
        let tolerance = 1;
        let mut best_key: Key = Matrix::zero();
        let mut best_score = 0;
        let mut iteration = 0;
        for a1 in 0..11 {
            for a2 in 0..11 {
                for a3 in 0..11 {
                    for a4 in 0..11 {
                        let key: Key = Matrix::new(&[a1, a2, a3, a4]);
                        let mut score = 0;
                        for point in self.data.iter() {
                            let success = point.try_key(&key, tolerance);
                            if success {
                                score += 1;
                            }
                        }
                        if score >= best_score {
                            best_key = key;
                            best_score = score;
                        }
                        if iteration % 1000 == 0 {
                            let percent = iteration as f64 / 14641.0 * 100.0;
                            println!("{percent:.0}% | best score {best_score}");
                        }
                        iteration += 1;
                    }
                }
            }
        }
        (best_key, best_score)
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
    let blue_set = DataSet::new(&blue_set);
    let red_set = DataSet::new(&red_set);
    println!("searching blue");
    let blue_result = blue_set.search();
    println!("blue result {:?}", blue_result);

    println!("searching red");
    let red_result = red_set.search();

    println!("Summary");
    println!("red result {:?} {}", red_result, red_set.data.len());
    println!("blue result {:?} {}", blue_result, blue_set.data.len());
}
