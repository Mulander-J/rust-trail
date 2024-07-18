pub fn square_of_sum(n: u32) -> u32 {
    //!Solution1
    // (1..=n).fold(0,|acc: u32, i| acc.saturating_add(i)).saturating_pow(2)
    //!Solution2
    // (1..=n).sum::<u32>().saturating_pow(2)
    //!Solution3
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    //!Solution1
    // (1..=n).fold(0,|acc, i| acc.saturating_add(i.saturating_pow(2)))
    //!Solution2
    // (1..=n).map(|x| x * x).sum()
    //!Solution3
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
