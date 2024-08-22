#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self(rank, file)),
            _ => None,
        }
        // if (0..8).contains(&rank) && (0..8).contains(&file) {
        //     Some(Self(rank, file))
        // } else {
        //     None
        // }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let (x1, y1) = (self.0.0, self.0.1);
        let (x2, y2) = (other.0.0, other.0.1);

        x1 == x2 || y1 == y2 || (x1 - x2).abs() == (y1 - y2).abs()
    }
}
