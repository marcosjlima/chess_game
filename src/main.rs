pub mod ui;

fn main() {
    let piece: ui::ChessPiece = ui::ChessPiece {
        position: ui::Position { row: 0, column: 0 },
        color: ui::Color::BLACK,
        icon: 'A',
    };

    let mut board = vec![vec![false; 8]; 8];

    board = ui::move_piece(piece, board);

    ui::print_board(board);
}
