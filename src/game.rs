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
    current_turn: Piece,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_turn: Piece::X,
            state: GameState::InPlay,
        }
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
}