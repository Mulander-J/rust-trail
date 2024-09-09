/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
const M: i32 = 26;
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// E(x) = (ai + b) mod m
/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match extended_gcd(a, M) {
        (x, _, _) if x == 1 => {
            Ok(plaintext
                .to_lowercase()
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_alphanumeric() {
                        Some(if c.is_numeric() {
                            c
                        } else {
                            // let n = (a * (((c as u8) - b'a') as i32) + b) % M;
                            // (b'a' + ((n + M) % M) as u8) as char

                            let n = (a * (((c as u8) - b'a') as i32) + b).rem_euclid(M);
                            (b'a' + n as u8) as char
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .chunks(5)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join(" "))
        }
        _ => Err(AffineCipherError::NotCoprime(a))
    }
}

/// D(y) = (a^-1)(y - b) mod m
/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    match extended_gcd(a, M) {
        (x, y, _) if x == 1 => {
            let inverse = y.rem_euclid(M);

            Ok(ciphertext
                .to_lowercase()
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_alphanumeric() {
                        Some(if c.is_numeric() {
                            c
                        } else {
                            // let n = (inverse * (((c as u8) - b'a') as i32 - b)) % M;
                            // (b'a' + ((n + M) % M) as u8) as char

                            let n = (inverse * (((c as u8) - b'a') as i32 - b)).rem_euclid(M);
                            (b'a' + n as u8) as char
                        })
                    } else { None }
                })
                .collect())
        }
        _ => Err(AffineCipherError::NotCoprime(a))
    }
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (d, x1, y1) = extended_gcd(b, a % b);
        (d, y1, x1 - (a / b) * y1)
    }
}