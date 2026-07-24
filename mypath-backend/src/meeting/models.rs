use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::shared::models::TimeSlot;

#[derive(Serialize, Deserialize, Debug)]
pub enum MeetingAttendees {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl MeetingAttendees {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::One => "One",
            Self::Two => "Two",
            Self::Three => "Three",
            Self::Four => "Four",
            Self::Five => "Five",
            Self::Six => "Six",
            Self::Seven => "Seven",
            Self::Eight => "Eight",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MeetingBookingInfo {
    pub date: NaiveDate,
    pub organizer_name: String,
    pub topic: String,
    pub contact: String,
    pub time_slot: TimeSlot,
    pub room_name: String,
    pub attendees: MeetingAttendees,
}
