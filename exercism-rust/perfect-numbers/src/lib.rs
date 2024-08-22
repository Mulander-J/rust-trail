use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match factors(num).iter().sum::<u64>().cmp(&num) {
        Ordering::Greater => Some(Classification::Abundant),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Less => Some(Classification::Deficient),
    }
}


fn factors(n: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let sqrt_n = (n as f64).sqrt() as u64;

    for i in 1..=sqrt_n {
        if n % i == 0 {
            if i != n {
                result.push(i);
            }
            if i != 1 && i != n / i {
                result.push(n / i);
            }
        }
    }
    result
}