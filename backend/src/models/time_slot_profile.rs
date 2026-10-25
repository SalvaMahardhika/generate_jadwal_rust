use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TimeSlotProfile {
    pub id: i64,
    pub academic_year_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub is_default: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}