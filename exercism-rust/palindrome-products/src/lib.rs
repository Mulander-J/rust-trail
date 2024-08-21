/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

fn is_palindrome(value: u64) -> bool {
    /* Solution2 */
    if value % 10 == 0 {
        return false;
    };
    let mut reverse = 0;
    let mut r = value;
    while r > 0 {
        reverse = reverse * 10 + r % 10;
        r /= 10;
    }
    value == reverse

    /* Solution1 */
    // let reversed: u64 = value
    //     .to_string()
    //     .chars()
    //     .rev()
    //     .collect::<String>()
    //     .parse()
    //     .unwrap();
}

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    /* Solution2 */
    (min..=max)
        .flat_map(|i| (i..=max).map(move |j| j * i))
        .fold(None, |mut res, current| {
            if is_palindrome(current) {
                let _insert = Palindrome::new(current).unwrap();
                match res {
                    None => res = Some((_insert, _insert)),
                    Some((min, max)) if min.into_inner() > current => res = Some((_insert, max)),
                    Some((min, max)) if max.into_inner() < current => res = Some((min, _insert)),
                    _ => {}
                };
            }
            res
        })

    /* Solution1 */
    // let mut products: Vec<_> = (min..=max)
    //     .flat_map(|n| (n..=max).map(move |j| j * n))
    //     .filter_map(Palindrome::new)
    //     .collect();
    //
    // if products.is_empty() {
    //     return None;
    // }
    //
    // products.sort_unstable_by_key(|p| p.into_inner());
    //
    // let min = products.first().cloned()?;
    // let max = products.last().cloned()?;
    //
    // Some((min, max))
}
