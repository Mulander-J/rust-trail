use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    /*
        Solution1
        TC:O(limit)
        SC:O(limit)
     */
    // let mut set = HashSet::new();
    // factors.iter().filter(|&&x| x > 0_u32).for_each(|&x| {
    //     let mut n = x;
    //     while n < limit {
    //         set.insert(n);
    //         n += x;
    //     }
    // });
    // set.iter().sum()


    /*
        Solution2
        TC:O(*factors.len())
        SC:O(limit)
     */
    // factors
    //     .iter()
    //     .filter(|&&x| x > 0)
    //     .flat_map(|&x| (x..limit).step_by(x as usize))
    //     .collect::<HashSet<_>>()
    //     .iter()
    //     .sum()

    /*
        Solution3
        TC:O(limit*factors.len())
        SC:O(1)
        适合大数limit
     */
    (1..limit)
    .filter(|i| factors.iter().any(|&f| f != 0 && i % f == 0))
    .sum()
}
