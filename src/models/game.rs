use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
#[allow(non_snake_case)]
pub struct GameModel {
    pub id: Uuid,
    pub field_name: String,
    pub address: String,
    pub day: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateGameSchema {
    pub field_name: String,
    pub address: String,
    pub day: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGameSchema {
    pub field_name: Option<String>,
    pub address: Option<String>,
    pub day: Option<String>
}