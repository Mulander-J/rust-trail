/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut i = 1;
    while n >1 {
        n = if n % 2 == 0 {
                n / 2
            } else {
                3 * n + 1
            };
        i += 1;
    };
    i
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}
