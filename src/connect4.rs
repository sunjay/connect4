const ROWS: usize = 6;
const COLS: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    PieceX,
    PieceO,
}

impl Default for Piece {
    fn default() -> Self {
        Piece::PieceX
    }
}

pub type Tile = Option<Piece>;

#[derive(Debug, Clone, Default)]
pub struct Connect4 {
    // Grid is stored *column-wise*
    columns: [[Tile; ROWS]; COLS],
    current_piece: Piece,
}

impl Connect4 {
    pub fn new() -> Connect4 {
        Connect4::default()
    }

    pub fn to_rows(&self) -> Vec<Vec<Tile>> {
        (0..self.rows()).map(|r| self.columns.iter().map(|col| col[r]).collect()).collect()
    }

    pub fn rows(&self) -> usize {
        self.columns[0].len()
    }

    pub fn cols(&self) -> usize {
        self.columns.len()
    }

    /// Drop the current piece into the given col
    pub fn drop_piece(&mut self, col: usize) {
    }
}
