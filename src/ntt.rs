use crate::common::Raisable;
use core::fmt::Debug;
use std::ops::{Add, Mul, Sub};

fn fft_recursive<
    ELEMENT: Debug
        + Clone
        + Raisable
        + Mul<Output = ELEMENT>
        + Add<Output = ELEMENT>
        + Sub<Output = ELEMENT>,
>(
    xs: &[ELEMENT],
    omega: ELEMENT,
) -> Vec<ELEMENT> {
    if xs.len() == 1 {
        return xs.to_vec();
    }
    let omega_sq = omega.clone() * omega.clone();
    let left = fft_recursive(
        &xs.iter().step_by(2).cloned().collect::<Vec<_>>(),
        omega_sq.clone(),
    );
    let right = fft_recursive(
        &xs.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>(),
        omega_sq,
    );
    let mut output = xs.to_vec();
    for (i, (x, y)) in left.iter().zip(right.iter()).enumerate() {
        let y_times_root = y.clone() * omega.clone().pow(i);
        output[i] = x.clone() + y_times_root.clone();
        output[i + left.len()] = x.clone() - y_times_root.clone();
    }
    output
}

/// Outputs the evaluation of polynomial a0+ a1x+ ... + a_(L-1)x^(L-1) on 1, omega, ..., omega^(L-1)
fn simple_fft<
    ELEMENT: Debug
        + Clone
        + Raisable
        + Mul<Output = ELEMENT>
        + Add<Output = ELEMENT>
        + Sub<Output = ELEMENT>,
>(
    xs: &mut [ELEMENT],
    omega: ELEMENT,
) {
    assert!(xs.len().is_power_of_two());
    let l = xs.len().ilog2();
    for i in 1..=l {
        let xi = omega.pow(1 << (i - 1));
        let m = 1 << (l - i);
        for j in 0..(1 << (i - 1)) {
            let t = 2 * j * m;
            for k in 0..m {
                let x_tk = xs[t + k].clone();
                let x_tkm = xs[t + k + m].clone();
                xs[t + k] = x_tk.clone() + x_tkm.clone();
                xs[t + k + m] = xi.pow(k) * (x_tk - x_tkm)
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::PrimeField;

    use super::*;

    #[test]
    fn test_fft_recursive() {
        // test case from https://vitalik.eth.limo/general/2019/05/12/fft.html
        let poly: Vec<PrimeField<337>> = [3, 1, 4, 1, 5, 9, 2, 6]
            .iter()
            .map(|i: &u64| PrimeField::new(*i))
            .collect();
        let omega = PrimeField::<337>::new(85);
        let evaluation: Vec<PrimeField<337>> = [31, 70, 109, 74, 334, 181, 232, 4]
            .iter()
            .map(|i: &u64| PrimeField::new(*i))
            .collect();
        assert_eq!(fft_recursive(&poly, omega.clone()), evaluation);
        let mut poly = poly.clone();
        simple_fft(&mut poly, omega);
        assert_eq!(poly, evaluation)
    }

    #[test]
    fn test_ntt() {
        let mut poly: Vec<PrimeField<17>> = (1..=8).map(|x: u64| x.into()).collect();
        // domain: 1, 2, 4, 8, 16, 15, 13, 9
        let omega: PrimeField<17> = 2.into();
        let evaluations = fft_recursive(&poly, omega.clone());

        simple_fft(&mut poly, omega);
        assert_eq!(poly, evaluations)
    }
}
