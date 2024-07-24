use num_bigint::BigUint;
use num_traits::ToPrimitive;

/// Pick a private key greater than 1 and less than {p}
pub fn private_key(p: u64) -> u64 {
    p - 1
}

/// Calculate public key using prime numbers {p} and {g}, and private key {a}
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_exponentiation(g, a, p)
}

/// Calculate secret key using prime number {p}, public key {b_pub}, and private key {a}
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_exponentiation(b_pub, a, p)
}

fn modular_exponentiation(base: u64, exp: u64, modulus: u64) -> u64 {
    /* Solution1 */
    // base.saturating_pow(exp as u32) % modulus

    /* Solution2 */
    // let mut e = exp;
    // let mut b = base;
    // let mut result = 1;
    // while e > 0 {
    //     if e % 2 == 1 {
    //         result = (result * b) % modulus as u128;
    //     }
    //     b = (b * b) % modulus as u128;
    //     e = e / 2;
    // }
    // result as u64

    /* Solution3 */
    // BigUint::from(base)
    // .modpow(&BigUint::from(exp), &BigUint::from(modulus))
    // .to_u64_digits()
    // .get(0)
    // .unwrap()
    // .clone()

    /* Solution3.1 */
    BigUint::from(base)
        .modpow(&BigUint::from(exp), &BigUint::from(modulus))
        .to_u64()
        .unwrap_or(0)
}
