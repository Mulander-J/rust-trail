pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        let text_len = chars.len();

        self.distribution(text_len)
            .iter()
            .flat_map(|row| row.iter().map(|&pos| chars[pos]))
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let chars: Vec<char> = cipher.chars().collect();
        let text_len = chars.len();

        self.distribution(text_len)
            .iter()
            .flatten()
            .zip(chars.iter())
            .fold(vec![' '; text_len], |mut acc, (&pos, &c)| {
                acc[pos] = c;
                acc
            })
            .iter()
            .collect()
    }

    /// Calculates the distribution of characters across the rails (rows).
    ///
    /// Given the length of the text `text_len` and the number of rails, this function
    /// returns a `Vec<Vec<usize>>` where each inner `Vec` represents a rail, and contains
    /// the indices of characters that belong to that rail.
    ///
    /// # Parameters
    ///
    /// * `text_len` - The length of the text to be distributed across the rails.
    ///
    /// # Returns
    ///
    /// A `Vec<Vec<usize>>` that contains the distribution of character indices across the rails.
    ///
    /// # Example
    ///
    /// Suppose `rails = 3` and `text_len = 10`, calling `distribution` will return:
    ///
    /// ```rust
    /// vec![
    ///     vec![0, 4, 8],
    ///     vec![1, 3, 5, 7, 9],
    ///     vec![2, 6]
    /// ];
    /// ```
    ///
    fn distribution(&self, text_len: usize) -> Vec<Vec<usize>> {
        let mut distribution: Vec<Vec<usize>> =
            vec![Vec::with_capacity(text_len / self.rails); self.rails];

        let cycle_len = 2 * (self.rails - 1);
        for i in 0..text_len {
            let pos_in_cycle = i % cycle_len;
            let row = if pos_in_cycle < self.rails {
                pos_in_cycle
            } else {
                cycle_len - pos_in_cycle
            };
            distribution[row].push(i);
        }

        distribution
    }
}
