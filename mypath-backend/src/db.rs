use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use std::str::FromStr;

pub async fn init_db(db_url: &str) -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            SqliteConnectOptions::from_str(db_url).unwrap().create_if_missing(true)
        ).await
        .unwrap();

    sqlx::query(
        r#"
        DROP TABLE IF EXISTS library_bookings;
        DROP TABLE IF EXISTS meeting_bookings;
        
        CREATE TABLE IF NOT EXISTS bookings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            booking_date DATE NOT NULL,
            time_slot VARCHAR(50) NOT NULL,
            booking_type VARCHAR(50) NOT NULL,
            contact VARCHAR(50) NOT NULL,
            first_name VARCHAR(255),
            last_name VARCHAR(255),
            attendees VARCHAR(50),
            organizer_name VARCHAR(255),
            topic VARCHAR(255),
            room_name VARCHAR(100),
            CONSTRAINT unique_booking_datetime UNIQUE(booking_date, time_slot, room_name)
        );

        CREATE TABLE IF NOT EXISTS master_classes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title VARCHAR(255) NOT NULL,
            description TEXT NOT NULL,
            timing VARCHAR(50) NOT NULL,
            date DATE NOT NULL,
            presenter_name VARCHAR(255) NOT NULL
        );

        CREATE TABLE IF NOT EXISTS master_class_enrollments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            master_class_id INTEGER NOT NULL,
            first_name VARCHAR(255) NOT NULL,
            last_name VARCHAR(255) NOT NULL,
            contact VARCHAR(50) NOT NULL
        );

        CREATE TABLE IF NOT EXISTS master_class_enquiries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            master_class_id INTEGER NOT NULL,
            contact VARCHAR(50) NOT NULL,
            message TEXT NOT NULL
        );
        "#
    )
    .execute(&pool).await
    .expect("Failed to create database tables");

    pool
}
