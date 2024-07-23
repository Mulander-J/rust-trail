use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    //! Solution1
    // match list.len() {
    //     0 => String::new(),
    //     1 => format!("And all for the want of a {}.", list[0]),
    //     l => {
    //         let head = list[0];
    //         list.iter()
    //             .enumerate()
    //             .map(|(i,x)| {
    //                 if i == l - 1 {
    //                     format!("And all for the want of a {head}.")
    //                 } else {
    //                     format!("For want of a {x} the {} was lost.", list[i + 1])
    //                 }
    //             })
    //             .collect::<Vec<_>>()
    //             .join("\n")
    //     }
    // }

    //! Solution2
    match list.len() {
        n if n <= 0 => String::new(),
        _ => list
            .windows(2)
            .map(|window| format!("For want of a {} the {} was lost.", window[0], window[1]))
            .chain(once(format!("And all for the want of a {}.", list[0]))) // 追加迭代+一次迭代
            .collect::<Vec<_>>()
            .join("\n"),
    }
}
