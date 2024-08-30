use std::iter::once;
use std::ops::Add;

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T: Copy + PartialOrd + Add<Output=T>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [a, b, c] = sides;
        let sum = a + b + c;
        let max_side = sides
            .iter()
            .fold(a, |acc, &n| if n > acc { n } else { acc });
        let double_max_side = max_side.add(max_side);

        // 如果三边之和大于两倍的最大边，则该三角形有效。
        if sum > double_max_side {
            Some(Self { sides })
        } else {
            None
        }
    }

    fn count_equal_pairs(&self) -> usize {
        // let mut count = 0;
        // for i in 0..self.sides.len() {
        //     if self.sides[i] == self.sides[(i + 1) % 3] {
        //         count += 1;
        //     }
        // }
        // count

        self.sides
            .iter()
            .chain(once(self.sides.first().unwrap()))
            .collect::<Vec<_>>()
            .windows(2)
            .fold(0, |mut acc, w| {
                if w[0] == w[1] {
                    acc += 1
                }
                acc
            })
    }

    pub fn is_scalene(&self) -> bool {
        self.count_equal_pairs() == 0
    }
    pub fn is_isosceles(&self) -> bool {
        // self.count_equal_pairs() == 1
        !self.is_scalene()
    }
    pub fn is_equilateral(&self) -> bool {
        self.count_equal_pairs() == 3
    }
}
