//! from https://www.di-mgt.com.au/lattice-lwe-simple-pke.html

use std::iter::repeat;

use crypto::{Matrix, PrimeField};
use rand::{thread_rng, Rng};

type F11 = PrimeField<11>;
type VecF11 = Matrix<5, 1, F11>;
type SecretKey = VecF11;
type PublicKey = (Matrix<5, 5, F11>, VecF11);
type Message = bool;
type Cipher = (VecF11, F11);

const Q: u64 = 11;

fn key_gen<R: Rng + ?Sized>(rng: &mut R) -> (PublicKey, SecretKey) {
    let sk: VecF11 = rng.gen();
    let a: Matrix<5, 5, F11> = rng.gen();
    let e: VecF11 = rng.gen();
    let b = a.clone() * sk.clone() + e;
    ((a, b), sk)
}

fn encrypt<R: Rng + ?Sized>(rng: &mut R, m: Message, pk: PublicKey) -> Cipher {
    let r: VecF11 = rng.gen();
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
    let mut rng = thread_rng();
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
