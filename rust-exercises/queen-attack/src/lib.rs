#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        let position_not_in_range = Self::check_position_not_in_range(rank, file);
        if position_not_in_range {
            return None
        }

        Some(ChessPosition{0: rank, 1: file})
    }

    fn check_position_not_in_range(rank: i32, file: i32) -> bool {
        let rank_not_in_range = (rank < 0) | (rank > 7);
        let file_not_in_range = (file < 0) | (file > 7);
        let position_not_in_range = rank_not_in_range | file_not_in_range;

        position_not_in_range
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen{position}
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
    //     check vertical
        let in_vertical: bool = Self::check_vertical(&self.position, &other.position);
    //     check horizontal
        let in_horizontal: bool = Self::check_horizontal(&self.position, &other.position);
    //     check diagonal
    //     check anti-diagonal
    }

    fn check_vertical(my_position: &ChessPosition, her_position: &ChessPosition) -> bool {
        my_position.1 == her_position.1
    }

    fn check_horizontal(my_position: &ChessPosition, her_position: &ChessPosition) -> bool {
        my_position.0 == her_position.0
    }
}
