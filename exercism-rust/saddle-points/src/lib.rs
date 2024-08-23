fn is_row_max(matrix: &[Vec<u64>], row: usize, current_max: u64) -> bool {
    for &n in &matrix[row] {
        if current_max < n {
            return false;
        }
    }
    true
}

fn is_col_min(matrix: &[Vec<u64>], col: usize, current_min: u64) -> bool {
    for n in matrix {
        if current_min > n[col] {
            return false;
        }
    }
    true
}

pub fn find_saddle_points(matrix: &[Vec<u64>]) -> Vec<(usize, usize)> {
    /* Solution1 */
    let mut saddle_points = Vec::new();
    let row_count = matrix.len();
    if row_count == 0 {
        return saddle_points;
    }
    let col_count = matrix[0].len();

    for i in 0..row_count {
        for j in 0..col_count {
            let current_value = matrix[i][j];
            if is_row_max(matrix, i, current_value) && is_col_min(matrix, j, current_value) {
                saddle_points.push((i, j));
            }
        }
    }

    saddle_points

    /* Solution2 */
    // let mut points: Vec<(usize, usize)> = vec![];
    // for (y, row) in input.iter().enumerate() {
    //     for (x, n) in row.iter().enumerate() {
    //         let col: Vec<u64> = input.iter().map(|v| v[x]).collect();
    //         if row.iter().all(|m| n>=m) && col.iter().all(|m| n<=m){
    //             points.push((y, x));
    //         }
    //     }
    // }
    // points
}