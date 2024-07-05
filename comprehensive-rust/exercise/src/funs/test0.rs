fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

#[test]
fn test_fib() {
    assert_eq!(fib(20), 6765);
}