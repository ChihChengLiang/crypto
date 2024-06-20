/// Round a/b
/// a/b + 1/2
pub fn round_frac2int(a: u64, b: u64) -> u64 {
    (2 * a + b) / (2 * b)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_round2int() {
        assert_eq!(round_frac2int(424999, 10000), 42);
        assert_eq!(round_frac2int(425, 10), 43);
    }
}
