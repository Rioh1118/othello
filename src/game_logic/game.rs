use std::cell::RefCell;
use crate::game_logic::board::Board;
use crate::game_logic::player::Player;
use crate::error::OthelloError;
use crate::game_logic::board::Position;
use crate::game_logic::rule::Rule;

pub enum GameState {
    InProgress,
    Finished {winner: Option<Player>}
}

pub struct Game {
    board: Board,
    current_player: RefCell<Player>,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_player: RefCell::new(Player::Black),
            state: GameState::InProgress,
        }
    }

    pub fn play_move(&self, position: Position) -> Result<(), OthelloError> {
        let current_player = *self.current_player.borrow();
        <Self as Rule>::apply_move(&self.board, current_player, position)?;

        let next_player = match current_player {
            Player::Black => Player::White,
            Player::White => Player::Black,
        };
        *(self.current_player.borrow_mut()) = next_player;
        Ok(())
    }
}