use std::iter;

const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

/* Solution2 */
pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size]; size];
    let mut movement = VECTORS.iter().cycle();
    let (mut x, mut y, mut n) = (-1, 0, 1..);

    for (move_x, move_y) in iter::once(size)
        .chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2))) // 按序生成每步的步数
        .flat_map(|steps| iter::repeat(movement.next().unwrap()).take(steps)) // 按顺时针方向为每步分配对应数量的方向向量
    {
        x += move_x;
        y += move_y;
        matrix[y as usize][x as usize] = n.next().unwrap();
    }

    matrix
}

/* Solution1 */
// pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
//     if size == 0 {
//         return vec![];
//     }
//
//
//     let mut matrix = vec![vec![0; size as usize]; size as usize];
//     let mut current_num = 1;
//     let mut left: usize = 0;
//     let mut right = (size - 1) as usize;
//     let mut top: usize = 0;
//     let mut bottom = (size - 1) as usize;
//
//     while left <= right && top <= bottom {
//         // 从左到右填充顶部行
//         for i in left..=right {
//             matrix[top][i] = current_num;
//             current_num += 1;
//         }
//         top += 1;
//
//         // 从上到下填充右边列
//         for i in top..=bottom {
//             matrix[i][right] = current_num;
//             current_num += 1;
//         }
//         right = right.saturating_sub(1);
//
//         if top <= bottom {
//             // 从右到左填充底部行
//             for i in (left..=right).rev() {
//                 matrix[bottom][i] = current_num;
//                 current_num += 1;
//             }
//             bottom = bottom.saturating_sub(1);
//         }
//
//         if left <= right {
//             // 从下到上填充左边列
//             for i in (top..=bottom).rev() {
//                 matrix[i][left] = current_num;
//                 current_num += 1;
//             }
//             left += 1;
//         }
//     }
//
//     matrix
// }
