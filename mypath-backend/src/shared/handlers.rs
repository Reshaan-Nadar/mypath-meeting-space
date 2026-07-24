use axum::Json;
use crate::shared::models::TimeSlot;

pub async fn get_time_slots() -> Json<Vec<TimeSlot>> {
    Json(TimeSlot::all_slots())
}
