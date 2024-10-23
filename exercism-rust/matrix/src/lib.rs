pub struct Matrix {
    row: Vec<Vec<u32>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self {
            row: input
                .lines()
                .map(|row| {
                    row.split_whitespace()
                        .filter(|str| !str.is_empty())
                        .map(|str| str.parse::<u32>().unwrap())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.row.get(row_no - 1).cloned()
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no == 0 || col_no > self.row[0].len() {
            return None;
        }

        Some(
            self.row
                .iter()
                .filter_map(|r| r.get(col_no - 1).cloned())
                .collect(),
        )
    }
}
