use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameSchema {
    pub date: chrono::NaiveDate,
    pub score_str: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGameSchema {
    pub date: Option<chrono::NaiveDate>,
    pub score_str: Option<String>,
}
