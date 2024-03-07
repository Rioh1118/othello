use serde::{Serialize, Deserialize};
use crate::schemas::board_info::BoardInfo;

#[derive(Serialize, Deserialize)]
pub struct MatchmakingRequest {
    pub player: PlayerInfo,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerInfo {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MatchmakingResponse {
    pub session_id: String,
    pub player: PlayerSessionInfo,
    pub opponent: PlayerSessionInfo,
    pub board: BoardInfo,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSessionInfo {
    pub id: String,
    pub color: String,
    pub name: String,
}


