//! from https://www.di-mgt.com.au/lattice-lwe-simple-pke.html

use crypto::{round_frac2int, Matrix, PrimeField};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::Normal;

const Q: u64 = 31;
// n
const KEY_SIZE: usize = 4;
// N
const NOISE_SIZE: usize = 7;
const SIGMA: f64 = 1.0;

type Fq = PrimeField<Q>;
type VecFq = Matrix<NOISE_SIZE, 1, Fq>;
type SecretKey = Matrix<KEY_SIZE, 1, Fq>;
type PublicKey = (Matrix<NOISE_SIZE, KEY_SIZE, Fq>, VecFq);
type Message = bool;
type Cipher = (Matrix<KEY_SIZE, 1, Fq>, Fq);

fn key_gen<R: Rng + ?Sized>(rng: &mut R) -> (PublicKey, SecretKey) {
    let sk: SecretKey = rng.gen();
    let a: Matrix<NOISE_SIZE, KEY_SIZE, Fq> = rng.gen();
    let normal = Normal::<f64>::new(0.0, SIGMA).unwrap();
    let e: VecFq = rng.sample(normal);
    println!("e {e}");
    let b = a.clone() * sk.clone() + e;
    ((a, b), sk)
}

fn encrypt<R: Rng + ?Sized>(rng: &mut R, m: Message, pk: PublicKey) -> Cipher {
    // Random binary vector
    let r: Matrix<NOISE_SIZE, 1, PrimeField<2>> = rng.gen();
    let r = r.map(|f2| f2.cast::<Q>());
    println!("r {r}");
    let (a, b) = pk;
    let u = a.t() * r.clone();
    let v = (b.t() * r)[(0, 0)].clone() + Fq::from((Q >> 1) * Into::<u64>::into(m));
    (u, v)
}

fn decrypt(c: Cipher, sk: SecretKey) -> Message {
    let (u, v) = c;
    let v_prime = (sk.t() * u)[(0, 0)].clone();
    let d = v - v_prime;
    let m = (round_frac2int(2 * d.get_n(), Q) % 2) == 1;
    m
}

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(123456);
    let (pk, sk) = key_gen(&mut rng);
    let (a, b) = pk.clone();
    println!("pk a {a} b {b} sk {sk}");
    let m = rng.gen();
    println!("message {}", m as u8);
    let cipher = encrypt(&mut rng, m, pk);
    let (u, v) = cipher.clone();
    println!("cipher u {u} v {v}");
    let m_decrypted = decrypt(cipher, sk);
    assert_eq!(m, m_decrypted)
}
