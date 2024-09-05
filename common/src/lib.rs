use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameInfo {
    pub game_id: uuid::Uuid,
    pub game_no: i32,
    pub score_str: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Game {
    pub date_id: uuid::Uuid,
    pub date: chrono::NaiveDate,
    #[serde(flatten)]
    pub game_info: GameInfo,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DateRecord {
    pub date_id: uuid::Uuid,
    pub date: chrono::NaiveDate,
    pub games: Vec<GameInfo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameResponse {
    pub status: String,
    pub data: Game,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordResponse {
    pub status: String,
    pub results: i32,
    pub dates: Vec<DateRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
