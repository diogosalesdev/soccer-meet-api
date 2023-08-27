use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub id: Uuid,
    pub field_name: String,
    pub address: String,
    pub day: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateGameSchema {
    pub field_name: String,
    pub address: String,
    pub day: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateGameSchema {
    pub field_name: Option<String>,
    pub address: Option<String>,
    pub day: Option<String>,
}
