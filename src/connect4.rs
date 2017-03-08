const ROWS: usize = 6;
const COLS: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidMove {
        column: usize,
    },
    NoSpaceLeftInColumn {
        column: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    PieceX,
    PieceO,
}

impl Piece {
    pub fn opposite(self) -> Piece {
        match self {
            Piece::PieceX => Piece::PieceO,
            Piece::PieceO => Piece::PieceX,
        }
    }
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

    pub fn current_piece(&self) -> Piece {
        self.current_piece
    }

    pub fn rows(&self) -> usize {
        self.columns[0].len()
    }

    pub fn cols(&self) -> usize {
        self.columns.len()
    }

    /// Drop the current piece into the given col
    pub fn drop_piece(&mut self, col: usize) -> Result<(), Error> {
        if col >= self.cols() {
            return Err(Error::InvalidMove {
                column: col,
            });
        }

        let index = self.columns[col].iter().rposition(|t| t.is_none());
        match index {
            Some(index) => self.columns[col][index] = Some(self.current_piece),
            None => return Err(Error::NoSpaceLeftInColumn {
                column: col,
            }),
        }

        self.current_piece = self.current_piece.opposite();

        Ok(())
    }
}
