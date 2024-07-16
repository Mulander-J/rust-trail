#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;

    match (_first_list.len(), _second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        // Learn [Windows](https://doc.rust-lang.org/std/slice/struct.Windows.html)
        (x, y) if x > y => {
            if _first_list.windows(y).any(|v| v == _second_list) {
                Superlist
            } else {
                Unequal
            }
        }
        (x, y) if x < y => {
            if _second_list.windows(x).any(|v| v == _first_list) {
                Sublist
            } else {
                Unequal
            }
        }
        (_, _) => {
            if _first_list == _second_list {
                Equal
            } else {
                Unequal
            }
        } 
        // (x, y) => {
        //     if x == y && is_strict_subsequence(_first_list,_second_list) {
        //         Equal
        //     } else if x > y && is_strict_subsequence(_first_list, _second_list) {
        //         Superlist
        //     } else if x< y && is_strict_subsequence(_second_list, _first_list) {
        //         Sublist
        //     } else {
        //         Unequal
        //     }
        // }
    }
}

// fn is_strict_subsequence<T: PartialEq>(long_list: &[T], short_list: &[T]) -> bool {
//     let (len1, len2) = (long_list.len(), short_list.len());
//
//     if len1 < len2 {
//         return  false
//     }
//
//     let mut i = 0;
//     let mut j = 0;
//
//     while i < len1 && j < len2 {
//         if long_list[i] == short_list[j] {
//             j += 1;
//         } else {
//             i -= j;
//             j = 0;
//         }
//         i += 1;
//     }
//
//     j == len2
// }
