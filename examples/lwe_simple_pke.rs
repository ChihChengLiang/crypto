//! from https://www.di-mgt.com.au/lattice-lwe-simple-pke.html

use std::iter::repeat;

use crypto::Zero;
use crypto::{Matrix, PrimeField};

type F11 = PrimeField<11>;
type VecF11 = Matrix<5, 1, F11>;
type SecretKey = VecF11;
type PublicKey = (Matrix<5, 5, F11>, VecF11);
type Message = bool;
type Cipher = (VecF11, F11);

const Q: u64 = 11;

fn key_gen() -> (PublicKey, SecretKey) {
    let sk: VecF11 = Matrix::new(&repeat(0).take(5).collect::<Vec<_>>());
    let a: Matrix<5, 5, F11> = Matrix::new(&repeat(0).take(25).collect::<Vec<_>>());
    let e: VecF11 = Matrix::new(&repeat(0).take(5).collect::<Vec<_>>());
    let b = a.clone() * sk.clone() + e;
    ((a, b), sk)
}

fn encrypt(m: Message, pk: PublicKey) -> Cipher {
    let r: VecF11 = Matrix::new(&repeat(0).take(5).collect::<Vec<_>>());
    let (a, b) = pk;
    let u = a.t() * r.clone();
    let v = (b.t() * r)[(0, 0)].clone() + F11::from((Q >> 1) * Into::<u64>::into(m));
    (u, v)
}

fn decrypt(c: Cipher, sk: SecretKey) -> Message {
    let (u, v) = c;
    let v_prime = (sk.t() * u)[(0, 0)].clone();
    let d = v - v_prime;
    let m = ((F11::from(2) * d).round_2int(Q) % 2) == 1;
    m
}

fn main() {
    let (pk, sk) = key_gen();
    let m = true;
    let cipher = encrypt(m, pk);
    let m_decrypted = decrypt(cipher, sk);
    assert_eq!(m, m_decrypted)
}
