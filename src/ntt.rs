use crate::common::Raisable;
use core::fmt::Debug;
use std::ops::{Add, Mul, Sub};

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
    fn test_ntt() {
        let mut poly: [PrimeField<11>; 4] = [1.into(), 2.into(), 3.into(), 4.into()];
        // domain: 1, 5, 3, 4
        let omega: PrimeField<11> = 5.into();
        // Failed why?
        let evaluations: [PrimeField<11>; 4] = [10.into(), 3.into(), 10.into(), 5.into()];

        simple_fft(&mut poly, omega);
        assert_eq!(poly, evaluations)
    }
}
