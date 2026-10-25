use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Subject {
    pub id: i64,
    pub subject_code: String,
    pub subject_name: String,
    pub color_code: String,
    pub category: String,
    pub passing_grade: i32,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}