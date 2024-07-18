pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => {
            format!(
                "{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",
                n.saturating_sub(1)
            )
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    //! Solution 1
    // let mut s = String::new();
    // let mut temp = String::new();
    // for i in end..=start {
    //     temp = verse(i);
    //     if i != end {
    //         temp.push_str("\n");
    //     }
    //     s.insert_str(0, &temp);
    // }
    // s

    //! Solution 2
    // let mut list = Vec::new();
    // for i in end..=start {
    //     list.push(verse(i))
    // }
    // let l:Vec<String> = list.iter().rev().cloned().collect();
    // l.join("\n")

    //! Solution 2.1
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
