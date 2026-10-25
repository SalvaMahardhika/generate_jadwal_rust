use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Class {
    pub id: i64,
    pub class_name: String,
    pub grade_id: i64,
    pub major_id: Option<i64>,
    pub academic_year_id: i64,
    pub time_slot_profile_id: Option<i64>,
    pub homeroom_teacher_id: Option<i64>,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}