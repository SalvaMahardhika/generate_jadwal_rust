use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ClassSubject {
    pub id: i64,
    pub class_id: i64,
    pub subject_id: i64,
    pub total_jp: i32,
    pub max_jp_per_day: i32,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}