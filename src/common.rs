pub trait Zero {
    fn zero() -> Self;
}
pub trait Raisable {
    fn pow(&self, power: usize) -> Self;
}
