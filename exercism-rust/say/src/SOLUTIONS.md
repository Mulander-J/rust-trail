# Say

## Solution1

```rust
const SMALL: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "ones", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const SCALE: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    match n {
        0..=19 => SMALL[n as usize].to_string(),
        20..=99 => {
            let ten = TENS[(n / 10) as usize];
            let rest = n % 10;
            if rest == 0 {
                ten.to_string()
            } else {
                format!("{}-{}", ten, SMALL[rest as usize])
            }
        }
        100..=999 => {
            let mut out = String::from(SMALL[n as usize / 100]);
            out.push_str(" hundred");
            let ones = n % 100;
            if ones > 0 {
                out.push(' ');
                out.push_str(&encode(ones));
            }
            out
        }
        1000..=u64::MAX => {
            let mut sets: Vec<u64> = Vec::new();
            let mut val = n;
            while val >= 1 {
                sets.push(val % 1000);
                val /= 1000;
            }
            let mut out = String::new();
            while let Some(modu) = sets.pop() {
                let len = sets.len();
                if modu == 0 {
                    continue;
                }
                if out.len() > 0 {
                    out.push(' ');
                }
                out.push_str(&encode(modu));
                if len > 0 {
                    out.push(' ');
                    out.push_str(SCALE[len]);
                }
            }
            out
        }
        _ => unreachable!(),
    }
}

```

## Solution2

```rust
pub fn encode(n: u64) -> String {
    let formatter =
        |num: u64, text: &str| format!("{} {} {}", encode(n / num), text, encode(n % num));
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        1..=99 => format!("{}-{}", encode(10 * (n / 10)), encode(n % 10)),
        1..=999 => formatter(100, "hundred"),
        1..=999_999 => formatter(1_000, "thousand"),
        1..=999_999_999 => formatter(1_000_000, "million"),
        1..=999_999_999_999 => formatter(1_000_000_000, "billion"),
        1..=999_999_999_999_999 => formatter(1_000_000_000_000, "trillion"),
        1..=999_999_999_999_999_999 => formatter(1_000_000_000_000_000, "quadrillion"),
        1..=u64::MAX => formatter(1_000_000_000_000_000_000, "quintillion"),
    }
        .replace(" zero", "")
}
```