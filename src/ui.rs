extern crate colour;

const M: usize = 8;
const N: usize = 8;

//printBoard(ChessPiece[][] pieces) {
pub fn print_board(_board: Vec<Vec<bool>>) {
    for i in 0..8 {
        print!("{} ", 8 - i);
        for _j in 0..8 {
            match _j % 2 {
                0 => print_piece(String::from("white"), false),
                _ => print_piece(String::from("black"), false),
            }
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


pub struct Position {
    pub row: usize,
    pub column: usize,
}

pub struct ChessPiece {
    pub position: Position,
    pub color: Color,
    pub icon: char
}

pub enum Color {
    WHITE,
    BLACK,
}

pub fn move_piece(piece: ChessPiece, mut board: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for i in 0..N {
        for j in 0..M {
            if piece.position.column == j && piece.position.row == i && board[i][j] == false {
                board[i][j] = true;
            }
        }
    }

    board
}