pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut factor = 2;
    while n > 1 {
        while n % factor == 0 {
            n /= factor;
            factors.push(factor);
        }
        factor += if factor == 2 { 1 } else { 2 } // skip even nums
    }
    factors
}