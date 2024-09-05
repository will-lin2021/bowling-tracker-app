use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct DateModel {
    pub date_id: Uuid,
    pub date: chrono::NaiveDate,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct GameModel {
    pub date_id: Uuid,
    pub game_id: Uuid,
    pub game_no: i32,
    pub score_str: String,
}
