pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    (3..)
        .step_by(2) // skip even numbers starting from 3
        .filter(|&x| (2..=(x as f64).sqrt() as u32).all(|a| x % a != 0)) // check is prime
        .nth((n - 1) as usize) // get nth
        .expect("Index out of bounds") // catch err
}
