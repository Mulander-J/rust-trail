use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F: Fn(T) -> bool + 'static, S: ToString>(matcher: F, subs: S) -> Matcher<T> {
        Self {
            matcher: Box::new(matcher),
            subs: subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: Rem<Output=T> + PartialEq + From<u8> + Copy + ToString> Fizzy<T> {
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// Map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I: Iterator<Item=T>>(self, iter: I) -> impl Iterator<Item=String>
    {
        iter.map(move |item| {
            let mut matchs = String::new();
            for matcher in &self.matchers {
                if (matcher.matcher)(item) {
                    matchs.push_str(&matcher.subs);
                }
            }
            if matchs.is_empty() {
                item.to_string()
            } else {
                matchs
            }
        })
    }
}

/// Convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: Rem<Output=T> + From<u8> + PartialEq + Copy + ToString>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"))
}
