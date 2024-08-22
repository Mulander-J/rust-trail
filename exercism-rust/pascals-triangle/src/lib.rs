pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        /* Solution2 */
        (0..self.0)
            .map(|i| (0..=i).map(|j| combination(i, j)).collect())
            .collect()

        /* Solution1 */
        // let mut ret: Vec<Vec<u32>> = Vec::with_capacity(self.0 as usize);
        //
        // for i in 0..self.0 as usize {
        //     let mut row: Vec<u32> = vec![1; i + 1];
        //     if i > 1 {
        //         for j in 1..i {
        //             row[j] = ret[i - 1][j - 1] + ret[i - 1][j];
        //         }
        //     }
        //     ret.push(row);
        // }
        //
        // ret
    }
}

/// C(n, k) = n! / (k! * (n-k)!)
fn combination(n: u32, k: u32) -> u32 {
    (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1))
}
