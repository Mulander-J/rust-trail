const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];
const SET_PLANT: fn(char) -> &'static str = |p| match p {
    'G' => "grass",
    'C' => "clover",
    'V' => "violets",
    'R' => "radishes",
    _ => unreachable!(),
};
pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    /* Solution2 */
    let cup_idx = STUDENTS.iter().position(|&stu| stu == student).unwrap() * 2;
    // let cup_idx = ((student.bytes().next().unwrap() - 'A' as u8) * 2) as usize;

    diagram
        .lines()
        .flat_map(|line| line[cup_idx..=cup_idx + 1].chars().map(SET_PLANT))
        .collect()

    /* Solution1 */
    // [
    //     "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    //     "Kincaid", "Larry",
    // ]
    //     .iter()
    //     .enumerate()
    //     .find_map(|(i, &stu)| {
    //         if stu == student {
    //             Some(
    //                 diagram
    //                     .lines()
    //                     // .split("\n")
    //                     // .collect::<Vec<&str>>()
    //                     // .iter()
    //                     .flat_map(|row| {
    //                         row[i * 2..=i * 2 + 1]
    //                             .chars()
    //                             .map(|x| match x {
    //                                 'G' => "grass",
    //                                 'C' => "clover",
    //                                 'V' => "violets",
    //                                 'R' => "radishes",
    //                                 _ => "",
    //                             })
    //                     })
    //                     .collect::<Vec<_>>(),
    //             )
    //         } else {
    //             None
    //         }
    //     })
    //     .unwrap_or(vec![])
}
