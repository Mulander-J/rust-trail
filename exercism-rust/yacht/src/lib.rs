#[derive(Debug, Eq, PartialEq)]
pub enum Category {
    Ones = 1,
    Twos = 2,
    Threes = 3,
    Fours = 4,
    Fives = 5,
    Sixes = 6,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let mut counts = [0_u8; 6];

    for &n in dice.iter() {
        counts[(n - 1) as usize] += 1;
    }

    match category {
        Category::Ones
        | Category::Twos
        | Category::Threes
        | Category::Fours
        | Category::Fives
        | Category::Sixes => {
            let num = category as u8;
            num * counts[(num - 1) as usize]
        }
        Category::FullHouse => {
            if counts.contains(&2) && counts.contains(&3) {
                (1..=6).map(|i| i * counts[(i - 1) as usize]).sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            if let Some(i) = (1..=6).find(|&i| counts[(i - 1) as usize] >= 4) {
                i * 4
            } else {
                0
            }
        }
        Category::BigStraight => {
            if counts == [0, 1, 1, 1, 1, 1] {
                30
            } else {
                0
            }
        }
        Category::LittleStraight => {
            if counts == [1, 1, 1, 1, 1, 0] {
                30
            } else {
                0
            }
        }
        Category::Choice => (1..=6).map(|i| i * counts[(i - 1) as usize]).sum(),
        Category::Yacht => {
            if counts.contains(&5) {
                50
            } else {
                0
            }
        }
    }
}
