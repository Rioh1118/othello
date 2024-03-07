use serde::{Serialize, Deserialize};
use crate::schemas::board_info::BoardInfo;

#[derive(Serialize, Deserialize)]
pub struct GameUpdate {
    pub session_id: String,
    pub board: BoardInfo,
    pub next_player_id: String,
    pub status: String,
    pub valid_moves: Vec<Move>,
}

#[derive(Serialize, Deserialize)]
pub struct Move {
    pub x: u8,
    pub y: u8,
}