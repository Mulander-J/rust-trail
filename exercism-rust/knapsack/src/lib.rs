#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut dp = vec![0; max_weight as usize + 1];

    for item in items {
        let weight = item.weight as usize;
        let value = item.value;

        for w in (weight..=max_weight as usize).rev() {
            dp[w] = dp[w].max(dp[w - weight] + value);
        }
    }

    dp[max_weight as usize]
}
