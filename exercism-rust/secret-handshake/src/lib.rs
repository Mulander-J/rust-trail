pub fn actions(n: u8) -> Vec<&'static str> {
    (0..=4).fold(vec![], |mut acc, i| {
        if n & (1 << i) > 0 {
            match i {
                0 => acc.push("wink"),
                1 => acc.push("double blink"),
                2 => acc.push("close your eyes"),
                3 => acc.push("jump"),
                4 => acc.reverse(),
                _ => {}
            }
        }
        acc
    })
}
