#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Position {
    pub fn new(row: usize, column: usize) -> Position {
        Position { row, column }
    }
    
    pub fn set_position(&mut self, row: usize, column: usize) {
        self.row = row;
        self.column = column;
    }
}
