use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Schedule {
    pub id: i64,
    pub academic_year_id: i64,
    pub class_id: i64,
    pub time_slot_id: i64,
    pub class_subject_teacher_id: Option<i64>,
    pub is_locked: bool,
    pub custom_activity: Option<String>,
    pub status: String,
    pub created_by: i64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}