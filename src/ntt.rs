fn naive_evaluate<const P: u64>(xs: &[u64], omega: u64) -> Vec<u64> {
    let mut output = vec![];
    for i in 0..xs.len() {
        let domain_i = omega.pow(i as u32) % P;
        let mut eval = 0;
        for x in xs.iter().rev() {
            eval = eval * domain_i % P;
            eval = (eval + x) % P;
        }
        output.push(eval)
    }
    output
}

fn fft_recursive<const P: u64>(xs: &[u64], omega: u64) -> Vec<u64> {
    if xs.len() == 1 {
        return xs.to_vec();
    }
    let omega_sq = omega * omega % P;
    let left = fft_recursive::<P>(&xs.iter().step_by(2).cloned().collect::<Vec<_>>(), omega_sq);
    let right = fft_recursive::<P>(
        &xs.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>(),
        omega_sq,
    );
    let mut output = xs.to_vec();
    for (i, (x, y)) in left.iter().zip(right.iter()).enumerate() {
        let y_times_root = y * omega.pow(i as u32) % P;
        output[i] = (x + y_times_root) % P;
        output[i + left.len()] = (x + P - y_times_root) % P;
    }
    output
}

/// Outputs the evaluation of polynomial a0+ a1x+ ... + a_(L-1)x^(L-1) on 1, omega, ..., omega^(L-1)
/// change is in-place and in [bit-reversal](https://en.wikipedia.org/wiki/Bit-reversal_permutation) order.
fn simple_fft<const P: u64>(xs: &mut [u64], omega: u64) {
    assert!(xs.len().is_power_of_two());
    let l = xs.len().ilog2();
    for i in 1..=l {
        let xi = omega.pow(1 << (i - 1)) % P;
        let m = 1 << (l - i);
        for j in 0..(1 << (i - 1)) {
            let t = 2 * j * m;
            for k in 0..m {
                let x_tk = xs[t + k];
                let x_tkm = xs[t + k + m];
                xs[t + k] = (x_tk + x_tkm) % P;
                xs[t + k + m] = xi.pow(k as u32) * (x_tk + P - x_tkm) % P
            }
        }
    }
}

fn bit_reversal<T: Clone>(xs: &[T]) -> Vec<T> {
    let mut output = xs.to_vec();
    let len = xs.len();
    assert!(len.is_power_of_two());
    let k = len.ilog2();
    for (i, x) in xs.iter().enumerate() {
        let reversed_i = i.reverse_bits() >> (usize::BITS - k);
        output[reversed_i] = x.clone();
    }
    output
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fft_recursive() {
        // test case from https://vitalik.eth.limo/general/2019/05/12/fft.html
        let poly = [3, 1, 4, 1, 5, 9, 2, 6];
        let omega = 85;
        let evaluation = [31, 70, 109, 74, 334, 181, 232, 4];
        assert_eq!(naive_evaluate::<337>(&poly, omega), evaluation);
        assert_eq!(fft_recursive::<337>(&poly, omega.clone()), evaluation);
        let mut poly = poly.clone();
        simple_fft::<337>(&mut poly, omega);
        assert_eq!(bit_reversal(&poly), evaluation)
    }

    #[test]
    fn test_ntt() {
        let mut poly = [1, 2, 3, 4, 5, 6, 7, 8];
        // domain: 1, 2, 4, 8, 16, 15, 13, 9
        // f(2) = [1 2  3  4  5  6  7 8] *
        //        [1 2  4  8 16 15 13 9]
        //  = sum([1 4 12 -2 -5  5  6 4]) = 8
        let omega = 2;
        let evaluation = fft_recursive::<17>(&poly, omega.clone());
        assert_eq!(naive_evaluate::<17>(&poly, omega), evaluation);
        simple_fft::<17>(&mut poly, omega);
        assert_eq!(bit_reversal(&poly), evaluation)
    }
}
