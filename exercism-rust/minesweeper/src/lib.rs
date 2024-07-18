pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    let cols = if rows > 0 { minefield[0].len() } else { 0 };
    let mut result = vec![vec!['0'; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            if minefield[r].as_bytes()[c] == b'*' {
                result[r][c] = '*';
                for i in (r.saturating_sub(1))..=(r + 1).min(rows - 1) {
                    for j in (c.saturating_sub(1))..=(c + 1).min(cols - 1) {
                        if result[i][j] != '*' {
                            result[i][j] = ((result[i][j] as u8) + 1) as char;
                        }
                    }
                }
            }
        }
    }

    result
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| if c == '0' { ' ' } else { c })
                .collect()
        })
        .collect()

    // minefield.iter().enumerate().map(|(i,r)| {
    //     r.char_indices().map(|(j,c)| {
    //         match c {
    //             '*' => c,
    //             _ => {
    //                 let mut count = 0;
    //                 for i in (i.saturating_sub(1))..=(i+1).min(rows -1) {
    //                     for j in (j.saturating_sub(1))..=(j+1).min(cols-1) {
    //                         if minefield[i].chars().nth(j).unwrap() =='*'{
    //                             count+=1;
    //                         }
    //                     }
    //                 }
    //                 if count > 0 {
    //                     (count + '0' as u8) as char
    //                 }else{
    //                     ' '
    //                 }
    //             }
    //         }
    //     }).collect()
    // }).collect()
}
