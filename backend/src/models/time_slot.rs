use chrono::{NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct TimeSlot {
    pub id: i64,
    pub time_slot_profile_id: i64,
    pub day: String,
    pub period_number: i32,
    pub label: Option<String>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_break: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}