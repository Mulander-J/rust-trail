// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn reverse(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
    fn rotate(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.d = self.d.rotate();
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.d = self.d.rotate().reverse();
        self
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Self { y: self.y + 1, ..self },
            Direction::South => Self { y: self.y - 1, ..self },
            Direction::East => Self { x: self.x + 1, ..self },
            Direction::West => Self { x: self.x - 1, ..self },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, c| match c {
            'R' => acc.turn_right(),
            'L' => acc.turn_left(),
            'A' => acc.advance(),
            _ => acc,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
