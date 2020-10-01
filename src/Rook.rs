use crate::board::Board;
use crate::piece::Piece;
use crate::position::Position;
use crate::{Color, NUM_COLS, NUM_ROWS};

#[derive(Debug)]
pub struct Rook {
    pub position: Position,
    pub color: Color,
}

impl Rook {
    pub fn new(row: usize, column: usize, color: Color) -> Rook {
        Rook {
            color,
            position: Position::new(row, column),
        }
    }

    pub fn set_position(&mut self, p: Position) {
        self.position = p;
    }

    pub fn icon(&self) -> String {
        String::from("R")
    }
}

impl Piece for Rook {
    fn possible_moves(&self, board: &Board) -> Vec<Vec<bool>> {
        let mut mat: Vec<Vec<bool>> = vec![vec![false; NUM_ROWS]; NUM_COLS];

        // Above
        let mut p = Position::new(self.position.row - 1, self.position.column);

        while board.position_exists(&p) && !board.there_is_a_piece(&p) {
            if p.row == 0 || p.column == 0 {
                break;
            }
            mat[p.row][p.column] = true;
            p.set_position(p.row - 1, p.column);
        }

        if board.position_exists(&p) && board.there_is_a_piece(&p) {
            mat[p.row][p.column] = true;
        }

        // Left
        p.set_position(self.position.row, self.position.column - 1);

        while board.position_exists(&p) && !board.there_is_a_piece(&p) {
            if p.row == 0 || p.column == 0 {
                break;
            }
            mat[p.row][p.column] = true;
            p.set_position(p.row, p.column - 1);
        }

        if board.position_exists(&p) && board.there_is_a_piece(&p) {
            mat[p.row][p.column] = true;
        }

        // Right
        p.set_position(self.position.row, self.position.column + 1);

        while board.position_exists(&p) && !board.there_is_a_piece(&p) {
            if p.row == 0 || p.column == 0 {
                break;
            }
            mat[p.row][p.column] = true;
            p.set_position(p.row, p.column + 1);
        }

        if board.position_exists(&p) && board.there_is_a_piece(&p) {
            mat[p.row][p.column] = true;
        }

        // Below
        p.set_position(self.position.row + 1, self.position.column);

        while board.position_exists(&p) && !board.there_is_a_piece(&p) {
            if p.row == 0 || p.column == 0 {
                break;
            }
            mat[p.row][p.column] = true;
            p.set_position(p.row + 1, p.column);
        }

        if board.position_exists(&p) && board.there_is_a_piece(&p) {
            mat[p.row][p.column] = true;
        }

        mat
    }
}

impl std::fmt::Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "R")
    }
}
