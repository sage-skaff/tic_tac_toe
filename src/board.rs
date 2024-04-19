// Path: src/board.rs

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    X,
    O,
    Empty,
}

pub struct Board {
    grid: [[Piece; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[Piece::Empty; 3]; 3],
        }
    }

    // Place a piece on the board
    pub fn place_piece(&mut self, row: usize, col: usize, piece: Piece) -> Result<(), &'static str> {
        if row >= 3 || col >= 3 {
            return Err("Position out of bounds");
        }
        if self.grid[row][col] != Piece::Empty {
            return Err("Position already occupied");
        }
        self.grid[row][col] = piece;
        Ok(())
    }

    // Get the board state as a string for CLI to display
    pub fn get_board_state(&self) -> String {
        self.grid.iter().map(|row| {
            row.iter().map(|&cell| match cell {
                Piece::X => 'X',
                Piece::O => 'O',
                Piece::Empty => ' ',
            }).collect::<String>()
        }).collect::<Vec<_>>().join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        let expected = "   \n   \n   ";  // Represents an empty board
        assert_eq!(board.get_board_state(), expected);
    }

    #[test]
    fn test_place_piece() {
        let mut board = Board::new();
        assert_eq!(board.place_piece(1, 1, Piece::X), Ok(()));
        assert_eq!(board.place_piece(0, 0, Piece::O), Ok(()));
        assert_eq!(board.place_piece(2, 2, Piece::X), Ok(()));
        let expected = "O  \n X \n  X";
        assert_eq!(board.get_board_state(), expected);
    }

    #[test]
    fn test_place_piece_out_of_bounds() {
        let mut board = Board::new();
        assert_eq!(board.place_piece(3, 0, Piece::X), Err("Position out of bounds"));
        assert_eq!(board.place_piece(0, 3, Piece::X), Err("Position out of bounds"));
    }

    #[test]
    fn test_overwrite_piece() {
        let mut board = Board::new();
        let _ = board.place_piece(0, 0, Piece::X);
        assert_eq!(board.place_piece(0, 0, Piece::O), Err("Position already occupied"));
    }

    #[test]
    fn test_get_board_state() {
        let mut board = Board::new();
        let _ = board.place_piece(0, 0, Piece::X);
        let _ = board.place_piece(1, 1, Piece::O);
        let _ = board.place_piece(2, 2, Piece::X);
        let expected = "X  \n O \n  X";
        assert_eq!(board.get_board_state(), expected);
    }
}

