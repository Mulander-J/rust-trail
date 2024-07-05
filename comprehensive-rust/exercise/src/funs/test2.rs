// Use an array such as the above to write a function transpose which will transpose a matrix (turn rows into columns):
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut res = [[0;3];3];
    for j in 0..3 {
        for k in 0..3 {
            res[j][k] = matrix[k][j];
        }   
    }
    res
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}