use crate::board::Board;
use crate::position::Position;

pub trait Piece {
	fn possible_moves(&self, board: &Board) -> Vec<Vec<bool>>;

	fn possible_move(&self, position: Position, board: &Board) -> bool {
		let pos = Piece::possible_moves(self, board);
		pos[position.row][position.column]
	}

	fn is_there_any_possible_move(&self, board: &Board) -> bool {
		let mat = Piece::possible_moves(self, board);

		for i in 0..mat.len() {
			for j in 0..mat.len() {
				if mat[i][j] {
					return true;
				}
			}
		}

		false
	}
}

// impl Debug for dyn Piece {
// 	fn fmt(&self, f: &mut Formatter) -> Result {
// 		write!(f, "Hi")
// 	}
// }

impl core::fmt::Debug for dyn Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Hi")
    }
}