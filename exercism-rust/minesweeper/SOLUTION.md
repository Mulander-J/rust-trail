# Solution

## 1. 按雷位更新周边计数

```rust
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();

    if rows == 0 {
        return vec![];
    }

    let cols = minefield[0].len();
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
}

```

## 2. 按位计算周边雷数

```rust
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();

    if rows == 0 {
        return vec![];
    }

    let cols = minefield[0].len();

    minefield.iter().enumerate().map(|(i,r)| {
        r.char_indices().map(|(j,c)| {
            match c {
                '*' => c,
                _ => {
                    let mut count = 0;
                    for i in (i.saturating_sub(1))..=(i+1).min(rows -1) {
                        for j in (j.saturating_sub(1))..=(j+1).min(cols-1) {
                            if minefield[i].chars().nth(j).unwrap() =='*'{
                                count+=1;
                            }
                        }
                    }
                    if count > 0 {
                        (count + '0' as u8) as char
                    }else{
                        ' '
                    }
                }
            }
        }).collect()
    }).collect()
}

```

## 2.1 基于函数式编程和迭代器风格

```rust
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let offset:Vec<_> = [-1,-1,-1,0,0,1,1,1].iter().zip([-1,0,1,-1,1,-1,0,1]).collect();
    let width = if minefield.len() > 0 { minefield[0].len() as i32 } else { 0 };
    let height = minefield.len() as i32;
    minefield.iter().enumerate().map(|(i, r)| {
        r.char_indices().map(|(j, c)| { match c {
            '*' => c,
             _ => match offset.iter().map(|&(n, m)| (i as i32 + n, j as i32 + m))
                .filter(|&(n, m)| n >= 0 && n < height && m >= 0 && m < width)
                .filter(|&(n, m)| minefield[n as usize].as_bytes()[m as usize] == b'*')
                .count() {
                    0 => ' ',
                    mines => (mines as u8 + '0' as u8) as char,
                }
        }}).collect()
    }).collect()
}
```