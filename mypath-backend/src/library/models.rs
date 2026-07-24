use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::shared::models::TimeSlot;

#[derive(Serialize, Deserialize, Debug)]
pub enum LibraryAttendees {
    One,
    Two,
    Three,
    Four,
}

impl LibraryAttendees {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::One => "One",
            Self::Two => "Two",
            Self::Three => "Three",
            Self::Four => "Four",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryBookingInfo {
    pub date: NaiveDate,
    pub first_name: String,
    pub last_name: String,
    pub contact: String,
    pub time_slot: TimeSlot,
    pub attendees: LibraryAttendees,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct BookingRecord {
    pub id: i64,
    pub booking_date: NaiveDate,
    pub time_slot: String,
    pub booking_type: String,
    pub contact: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub attendees: Option<String>,
    pub organizer_name: Option<String>,
    pub topic: Option<String>,
    pub room_name: Option<String>,
}
