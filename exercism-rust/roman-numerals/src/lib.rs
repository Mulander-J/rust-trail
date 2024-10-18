use std::fmt::{Display, Formatter, Result};

// I II III IV V VI VII VIII IX X
// XI XII XIII XIV XV XVI XVII XVIII XIX XX
const RM_NUMS: [(u32, &str); 13] = [
    (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
    (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
    (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
];

pub struct Roman {
    num: u32,
}

impl Roman {
    fn get_roman(&self) -> String {
        let mut result = String::new();
        let mut num = self.num;

        for &(value, symbol) in &RM_NUMS {
            while num >= value {
                result.push_str(symbol);
                num -= value;
            }
        }
        result
    }
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.get_roman())
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Self { num }
    }
}
