pub fn answer(command: &str) -> Option<i32> {
    // unknown_operation
    if command.trim_end()
        .get(command.len().saturating_sub(2)..command.len().saturating_sub(1))
        .unwrap()
        .chars()
        .all(|c| !c.is_numeric())
    {
        return None;
    }

    // valid iter
    let mut parts = command
        .split(|c: char| c == '?' || c.is_whitespace())
        .filter(|&word| {
            !word.is_empty()
                && (["plus", "minus", "multiplied", "divided"].contains(&word)
                || word.chars().all(|c| c.is_numeric() || c == '-'))
        });

    // miss opt
    if parts.clone().count() % 2 == 0 {
        return None;
    }

    let mut result = parts.next()?.parse().ok()?;
    let mut opr = "";
    while let Some(part) = parts.next() {
        if ["plus", "minus", "multiplied", "divided"].contains(&part) {
            opr = part;
        } else {
            let num: i32 = part.parse().ok()?;
            result = match opr {
                "plus" => result + num,
                "minus" => result - num,
                "multiplied" => result * num,
                "divided" => {
                    if num == 0 {
                        return None; // watch zero divided
                    }
                    result / num
                }
                _ => return None,
            };
        }
    }

    Some(result)
}
