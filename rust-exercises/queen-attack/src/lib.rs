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
        let in_diagonal: bool = Self::check_diagonal(&self.position, &other.position);
    //     check anti-diagonal
        let in_anti_diagonal: bool = Self::check_anti_diagonal(&self.position, &other.position);

        in_vertical | in_horizontal | in_diagonal | in_anti_diagonal
    }

    fn check_vertical(my_position: &ChessPosition, other_position: &ChessPosition) -> bool {
        my_position.1 == other_position.1
    }

    fn check_horizontal(my_position: &ChessPosition, other_position: &ChessPosition) -> bool {
        my_position.0 == other_position.0
    }

    fn check_diagonal(my_position: &ChessPosition, other_position: &ChessPosition) -> bool {
        let my_diag_value = my_position.0 - my_position.1;
        let other_diag_value = other_position.0 - other_position.1;

        my_diag_value == other_diag_value
    }

    fn check_anti_diagonal(my_position: &ChessPosition, other_position: &ChessPosition) -> bool {
        let my_anti_diag_value = my_position.0 + my_position.1;
        let other_anti_diag_value = other_position.0 + other_position.1;

        my_anti_diag_value == other_anti_diag_value
    }
}
