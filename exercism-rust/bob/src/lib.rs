pub fn reply(message: &str) -> &str {
    /*
      Solution1
      注意 空集的all会返回true
    */
    // match (
    //     message.trim().is_empty(),     // is_empty
    //     message.trim().ends_with('?'), // question mark
    //     message
    //         .trim()
    //         .chars()
    //         .filter(|x| x.is_ascii_alphabetic())
    //         .collect::<Vec<_>>()
    //         .is_empty(), // empty
    //     message
    //         .trim()
    //         .chars()
    //         .filter(|x| x.is_ascii_alphabetic())
    //         .all(|x| x.is_uppercase()), // uppercase
    // ) {
    //     (true, _, _, _) => "Fine. Be that way!",       // whitespace
    //     (_, true, _, false) => "Sure.",                // question mark
    //     (_, true, true, true) => "Sure.",              // question mark
    //     (_, false, false, true) => "Whoa, chill out!", // uppercase alphabetic
    //     (_, true, false, true) => "Calm down, I know what I'm doing!", // uppercase alphabetic + question mark
    //     _ => "Whatever.",
    // }

    /* Solution1.1 */
    // let trimmed = message.trim();
    // let alphabetic_chars: Vec<_> = trimmed.chars().filter(|c| c.is_ascii_alphabetic()).collect();
    // let is_empty = trimmed.is_empty();
    // let is_question = trimmed.ends_with('?');
    // let no_alphabetic = alphabetic_chars.is_empty();
    // let all_uppercase = alphabetic_chars.iter().all(|c| c.is_uppercase());
    //
    // match (is_empty, is_question, no_alphabetic, all_uppercase) {
    //     (true, _, _, _) => "Fine. Be that way!", // whitespace
    //     (_, true, _, false) => "Sure.", // question mark
    //     (_, true, true, true) => "Sure.", // question mark
    //     (_, false, false, true) => "Whoa, chill out!", // uppercase alphabetic
    //     (_, true, false, true) => "Calm down, I know what I'm doing!", // uppercase alphabetic + question mark
    //     _ => "Whatever.",
    // }

    /* Solution2 */
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }
    let is_question = trimmed.ends_with('?');
    // let is_yell = trimmed.chars().any(|x|x.is_alphabetic()) && !trimmed.chars().any(|x|x.is_lowercase());
    let is_yell = trimmed.contains(char::is_alphabetic) && !trimmed.contains(char::is_lowercase);

    match (is_question, is_yell) {
        (true, false) => "Sure.",                            // question mark
        (false, true) => "Whoa, chill out!",                 // uppercase alphabetic
        (true, true) => "Calm down, I know what I'm doing!", // uppercase alphabetic + question mark
        _ => "Whatever.",
    }
}
