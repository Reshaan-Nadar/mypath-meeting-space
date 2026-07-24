use axum::{extract::State, http::StatusCode, Json};
use crate::state::AppState;
use crate::masterclass::models::{MasterClassCreatePayload, MasterClassEnrollPayload, MasterClassEnquirePayload, MasterClassRecord};

pub async fn masterclass_create(
    State(state): State<AppState>,
    Json(payload): Json<MasterClassCreatePayload>
) -> Result<StatusCode, StatusCode> {
    let expected_hash = std::env::var("ADMIN_HASH").unwrap_or_else(|_| "secret_hash".to_string());
    if payload.admin_hash != expected_hash {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let result = sqlx::query(
        r#"
        INSERT INTO master_classes (
            title, description, timing, date, presenter_name
        )
        VALUES (?, ?, ?, ?, ?)
        "#
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.timing)
    .bind(payload.date)
    .bind(&payload.presenter_name)
    .execute(&state.db).await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("❌ DB Error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn masterclass_list(State(state): State<AppState>) -> Result<Json<Vec<MasterClassRecord>>, StatusCode> {
    let records = sqlx::query_as::<_, MasterClassRecord>("SELECT * FROM master_classes")
        .fetch_all(&state.db).await;

    match records {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            eprintln!("❌ DB Error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn masterclass_enroll(
    State(state): State<AppState>,
    Json(payload): Json<MasterClassEnrollPayload>
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        r#"
        INSERT INTO master_class_enrollments (
            master_class_id, first_name, last_name, contact
        )
        VALUES (?, ?, ?, ?)
        "#
    )
    .bind(payload.master_class_id)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.contact)
    .execute(&state.db).await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("❌ DB Error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn masterclass_enquire(
    State(state): State<AppState>,
    Json(payload): Json<MasterClassEnquirePayload>
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query(
        r#"
        INSERT INTO master_class_enquiries (
            master_class_id, contact, message
        )
        VALUES (?, ?, ?)
        "#
    )
    .bind(payload.master_class_id)
    .bind(&payload.contact)
    .bind(&payload.message)
    .execute(&state.db).await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            eprintln!("❌ DB Error: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
