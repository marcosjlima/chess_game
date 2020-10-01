extern crate colour;
use crate::board::Board;
use crate::{NUM_COLS, NUM_ROWS};

//printBoard(ChessPiece[][] pieces) {
pub fn print_board(_board: &Board) {
    for i in 0..NUM_ROWS {
        print!("{} ", NUM_ROWS - i);
        for _j in 0..NUM_COLS {
            print_piece(String::from(""), false);
        }
        println!();
    }
    println!("  A B C D E F G H");
}

//pub print_piece(ChessPiece piece, boolean background) {
pub fn print_piece(piece: String, background: bool) {
    if background {
        colour::blue!(" ");
    }

    match piece.as_str() {
        "white" => colour::white!("-"),
        "black" => colour::red!("-"),
        _ => print!("-"),
    }

    print!(" ");
}
