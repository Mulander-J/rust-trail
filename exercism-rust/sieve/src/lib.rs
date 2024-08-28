pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    /* Solution1 */
    // (1..=upper_bound).filter(is_prime).collect()

    /* Solution2 */
    // (2..=upper_bound).fold(vec![], |mut primes, candidate| {
    //     if primes.iter().all(|prime| candidate % prime != 0) {
    //         primes.push(candidate);
    //     }
    //     primes
    // })

    /* Solution3 */
    let mut numbers: Vec<_> = (2..=upper_bound).map(Option::from).collect();
    (0..numbers.len())
        .filter_map(|i| {
            let prime = numbers[i].take()?;
            (prime..=upper_bound).step_by(prime as usize)
                .skip(1)
                .for_each(|i| numbers[(i - 2) as usize] = None);
            Some(prime)
        })
        .collect()
}
fn is_prime(num: &u64) -> bool {
    let n = *num;
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
