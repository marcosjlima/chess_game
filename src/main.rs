use chess_game::bishop::Bishop;
use chess_game::board::Board;
use chess_game::ui;
use chess_game::Color;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut board: Board = Board {
        bishops: Vec::new(),
    };

    board.bishops.push(Bishop::new(1, 1, Color::BLACK));
    board.bishops.push(Bishop::new(1, 2, Color::BLACK));
    board.bishops.push(Bishop::new(1, 3, Color::BLACK));

    println!("{:#?}", board);

    ui::print_board(&board);

    Ok(())
}
