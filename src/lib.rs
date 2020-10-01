pub mod bishop;
pub mod board;
pub mod piece;
pub mod position;
pub mod queen;
pub mod rook;
pub mod ui;

pub const NUM_ROWS: usize = 8;
pub const NUM_COLS: usize = 8;

#[derive(Debug)]
pub enum Color {
    WHITE,
    BLACK,
}
