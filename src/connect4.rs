pub const ROWS: usize = 6;
pub const COLS: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    InvalidMove,
    NoSpaceLeftInColumn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    PieceX,
    PieceO,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    WinnerX,
    WinnerO,
    Tie,
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
    winner: Option<Winner>,
}

impl Connect4 {
    pub fn new() -> Connect4 {
        Connect4::default()
    }

    pub fn to_rows(&self) -> Vec<Vec<Tile>> {
        (0..ROWS).map(|r| self.columns.iter().map(|col| col[r]).collect()).collect()
    }

    pub fn winner(&self) -> Option<Winner> {
        self.winner
    }

    pub fn current_piece(&self) -> Piece {
        self.current_piece
    }

    /// Drop the current piece into the given col
    pub fn drop_piece(&mut self, col: usize) -> Result<(), Error> {
        if self.winner.is_some() || col >= COLS {
            return Err(Error::InvalidMove);
        }

        let index = self.columns[col].iter().rposition(|t| t.is_none());
        match index {
            Some(index) => self.columns[col][index] = Some(self.current_piece),
            None => return Err(Error::NoSpaceLeftInColumn),
        }

        self.current_piece = self.current_piece.opposite();

        self.winner = self.check_winner();

        Ok(())
    }

    fn check_winner(&self) -> Option<Winner> {
        for (i, col) in self.columns.iter().enumerate() {
            // We only need to check the top piece in each column (if any) because
            // any winner will be connected to one of those pieces
            let first_piece = col.iter().position(|t| t.is_some());
            if let Some(first_piece) = first_piece {
                let winner = self.search_tile(i, first_piece);
                if winner.is_some() {
                    return winner;
                }
            }
        }

        if self.is_full() {
            Some(Winner::Tie)
        }
        else {
            None
        }
    }

    fn search_tile(&self, col: usize, first_piece: usize) -> Option<Winner> {
        use self::Direction::*;

        let piece = self.columns[col][first_piece].unwrap();

        // count of this piece in all 8 directions
        let mut directions = [1usize; 8];
        // 4 because the game is called Connect 4
        for steps in 1..4 {
            for (i, dir) in [N, NE, E, SE, S, SW, W, NW].into_iter().enumerate() {
                if self.test_direction(piece, col, first_piece, steps, *dir) {
                    directions[i] += 1;
                }
            }
        }

        for found in directions.into_iter() {
            if *found == 4 {
                return match piece {
                    Piece::PieceX => Some(Winner::WinnerX),
                    Piece::PieceO => Some(Winner::WinnerO),
                };
            }
        }

        None
    }

    fn test_direction(&self, piece: Piece, col: usize, row: usize, steps: usize, dir: Direction) -> bool {
        use self::Direction::*;

        let (next_col, next_row) = match dir {
            N if row >= steps => (col, row - steps),
            NE if row >= steps && col < (COLS - steps) => (col + steps, row - steps),
            E if col < (COLS - steps) => (col + steps, row),
            SE if row < (ROWS - steps) && col < (COLS - steps) => (col + steps, row + steps),
            S if row < (ROWS - steps) => (col, row + steps),
            SW if row < (ROWS - steps) && col >= steps => (col - steps, row + steps),
            W if col >= steps => (col - steps, row),
            NW if row >= steps && col >= steps => (col - steps, row - steps),
            _ => return false,
        };

        self.columns[next_col][next_row] == Some(piece)
    }

    fn is_full(&self) -> bool {
        self.columns.iter().all(|col| col.iter().all(|tile| tile.is_some()))
    }
}
