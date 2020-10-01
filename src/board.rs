use crate::bishop::Bishop;
use crate::position::Position;
use crate::{NUM_COLS, NUM_ROWS};

#[derive(Debug)]
pub struct Board {
    pub bishops: Vec<Bishop>,
}

impl Board {
    pub fn position_exists(&self, position: &Position) -> bool {
        position.row < NUM_ROWS && position.column < NUM_COLS
        // position.row >= 0
        // && position.row < NUM_ROWS
        // && position.column >= 0
        // && position.column < NUM_COLS
    }
    pub fn there_is_a_piece(&self, position: &Position) -> bool {
        let mut r = false;

        for i in 0..self.bishops.len() {
            r = self.bishops[i].position.row == position.row
                && self.bishops[i].position.column == position.column;
                if r{
                    break;
                }
        }

        r
    }
}
