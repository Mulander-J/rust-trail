pub fn raindrops(n: u32) -> String {
    let result = [(3, "Pling"), (5, "Plang"), (7, "Plong")].iter().fold(
        String::new(),
        |mut acc, &(factor, sound)| {
            if n % factor == 0 {
                acc.push_str(sound);
            }
            acc
        },
    );

    if result.is_empty() {
        n.to_string()
    } else {
        result
    }
}
