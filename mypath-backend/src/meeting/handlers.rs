use axum::{extract::{Path, State}, http::StatusCode, Json};
use crate::state::AppState;
use crate::meeting::models::MeetingBookingInfo;
use crate::library::models::BookingRecord; // Since they share the same table schema and output model

pub async fn meeting_book_add(
    State(state): State<AppState>,
    Json(payload): Json<MeetingBookingInfo>
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        r#"
        INSERT INTO bookings (
            booking_date, time_slot, booking_type, contact,
            organizer_name, topic, room_name, attendees
        )
        VALUES (?, ?, 'Meeting', ?, ?, ?, ?, ?)
        "#
    )
    .bind(payload.date)
    .bind(payload.time_slot.as_str())
    .bind(&payload.contact)
    .bind(&payload.organizer_name)
    .bind(&payload.topic)
    .bind(&payload.room_name)
    .bind(payload.attendees.as_str())
    .execute(&state.db).await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(StatusCode::CONFLICT)
        }
        Err(e) => {
            eprintln!("❌ DB Error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn meeting_book_remove(
    State(state): State<AppState>,
    Path(id): Path<i64>
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM bookings WHERE id = ? AND booking_type = 'Meeting'")
        .bind(id)
        .execute(&state.db).await;

    match result {
        Ok(res) if res.rows_affected() > 0 => Ok(StatusCode::OK),
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            eprintln!("❌ DB Error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn meeting_book_list(State(state): State<AppState>) -> Result<Json<Vec<BookingRecord>>, StatusCode> {
    let records = sqlx::query_as::<_, BookingRecord>("SELECT * FROM bookings WHERE booking_type = 'Meeting'")
        .fetch_all(&state.db).await;

    match records {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            eprintln!("❌ DB Error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
