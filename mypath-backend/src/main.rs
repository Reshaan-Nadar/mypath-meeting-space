#![warn(clippy::pedantic)]
#![warn(clippy::all)]

pub mod db;
pub mod library;
pub mod masterclass;
pub mod meeting;
pub mod shared;
pub mod state;

use axum::{ routing::{ delete, get, post }, Router };
use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use tower_http::cors::{ Any, CorsLayer };

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://sqlite.db".to_string());

    let pool = db::init_db(&db_url).await;
    let state = state::AppState { db: pool };
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8090);

    let app = Router::new()
        .route("/library/book/add", post(library::handlers::library_book_add))
        .route("/library/book/remove/{id}", delete(library::handlers::library_book_remove))
        .route("/library/book/list", get(library::handlers::library_book_list))
        .route("/meeting/book/add", post(meeting::handlers::meeting_book_add))
        .route("/meeting/book/remove/{id}", delete(meeting::handlers::meeting_book_remove))
        .route("/meeting/book/list", get(meeting::handlers::meeting_book_list))
        .route("/shared/slots", get(shared::handlers::get_time_slots))
        .route("/masterclass/create", post(masterclass::handlers::masterclass_create))
        .route("/masterclass/list", get(masterclass::handlers::masterclass_list))
        .route("/masterclass/enroll", post(masterclass::handlers::masterclass_enroll))
        .route("/masterclass/enquire", post(masterclass::handlers::masterclass_enquire))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any)
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 Server running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use axum::{ body::Body, http::{ Request, StatusCode } };
    use tower::ServiceExt;
    use super::*;

    #[tokio::test]
    async fn test_shared_slots_route() {
        let pool = db::init_db("sqlite::memory:").await;
        let state = state::AppState { db: pool };

        let app = Router::new()
            .route("/shared/slots", get(shared::handlers::get_time_slots))
            .with_state(state);

        let response = app
            .oneshot(Request::builder().uri("/shared/slots").body(Body::empty()).unwrap()).await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
