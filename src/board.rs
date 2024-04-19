// Path: src/board.rs

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    X,
    O,
    Empty,
}

pub struct Board {
    pub grid: [[Piece; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[Piece::Empty; 3]; 3],
        }
    }

    // Place a piece on the board
    pub fn place_piece(
        &mut self,
        row: usize,
        col: usize,
        piece: Piece,
    ) -> Result<(), &'static str> {
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
        self.grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&cell| match cell {
                        Piece::X => "X".to_string(),
                        Piece::O => "O".to_string(),
                        Piece::Empty => " ".to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join(" | ")
            })
            .collect::<Vec<String>>()
            .join("\n---------\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_initializes_empty() {
        let board = Board::new();
        for row in board.grid.iter() {
            for &cell in row {
                assert_eq!(cell, Piece::Empty);
            }
        }
    }

    #[test]
    fn placing_pieces_on_board() {
        let mut board = Board::new();
        assert_eq!(board.grid[0][0], Piece::Empty); // Ensure initial state is empty
        
        board.place_piece(0, 0, Piece::X).unwrap();
        assert_eq!(board.grid[0][0], Piece::X); // Check if piece was placed
    }

    #[test]
    fn test_new_board() {
        let board = Board::new();
        let expected = "  |   |  \n---------\n  |   |  \n---------\n  |   |  ";
        assert_eq!(board.get_board_state(), expected);
    }

    #[test]
    fn test_place_piece() {
        let mut board = Board::new();
        board.place_piece(0, 0, Piece::O).unwrap();
        board.place_piece(1, 1, Piece::X).unwrap();
        board.place_piece(2, 2, Piece::X).unwrap();
        let expected = "O |   |  \n---------\n  | X |  \n---------\n  |   | X";
        assert_eq!(board.get_board_state(), expected);
    }

    #[test]
    fn test_get_board_state() {
        let mut board = Board::new();
        board.place_piece(0, 0, Piece::X).unwrap();
        board.place_piece(1, 1, Piece::O).unwrap();
        board.place_piece(2, 2, Piece::X).unwrap();
        let expected = "X |   |  \n---------\n  | O |  \n---------\n  |   | X";
        assert_eq!(board.get_board_state(), expected);
    }
}
