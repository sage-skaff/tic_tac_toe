// Path: src/game.rs

use crate::board::{Board, Piece};

#[derive(Debug, PartialEq)]
pub enum GameState {
    InPlay,
    Win(Piece),
    Draw,
}

pub struct Game {
    pub board: Board,
    pub current_turn: Piece,
    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_turn: Piece::X,
            state: GameState::InPlay,
        }
    }

    pub fn play_move(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if let GameState::InPlay = self.state {
            self.board.place_piece(row, col, self.current_turn)?;
            self.update_game_state(row, col);
            self.switch_turn();
            Ok(())
        } else {
            Err("Game is not in play")
        }
    }

    fn switch_turn(&mut self) {
        self.current_turn = match self.current_turn {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
            _ => unreachable!(),
        };
    }

    fn update_game_state(&mut self, row: usize, col: usize) {
        // This function should check if the last move won the game or if the board is full
        if self.has_won(row, col) {
            self.state = GameState::Win(self.current_turn);
        } else if self.is_draw() {
            self.state = GameState::Draw;
        }
    }

    fn has_won(&self, row: usize, col: usize) -> bool {
        // ToDo: Add logic to check if the player has won
        false
    }

    fn is_draw(&self) -> bool {
        // Check if all fields on the board are occupied and no one has won yet
        self.board.grid.iter().all(|row| row.iter().all(|&cell| cell != Piece::Empty))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_initializes_correctly() {
        let game = Game::new();
        assert_eq!(game.current_turn, Piece::X);
        assert_eq!(game.state, GameState::InPlay);
    }

    #[test]
    fn test_play_moves() {
        let mut game = Game::new();
        assert_eq!(game.play_move(0, 0), Ok(()));
        assert_eq!(game.current_turn, Piece::O);
        assert_eq!(game.play_move(0, 0), Err("Position already occupied"));
        assert_eq!(game.play_move(1, 1), Ok(()));
        assert_eq!(game.current_turn, Piece::X);
    }
}