use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum TimeSlot {
    #[serde(rename = "09:00-10:00")] NineToTen,
    #[serde(rename = "10:00-11:00")] TenToEleven,
    #[serde(rename = "11:00-12:00")] ElevenToTwelve,
    #[serde(rename = "12:00-13:00")] TwelveToThirteen,
    #[serde(rename = "13:00-14:00")] ThirteenToFourteen,
    #[serde(rename = "14:00-15:00")] FourteenToFifteen,
    #[serde(rename = "15:00-16:00")] FifteenToSixteen,
    #[serde(rename = "16:00-17:00")] SixteenToSeventeen,
    #[serde(rename = "17:00-18:00")] SeventeenToEighteen,
    #[serde(rename = "18:00-19:00")] EighteenToNineteen,
}

impl TimeSlot {
    pub fn all_slots() -> Vec<TimeSlot> {
        vec![
            TimeSlot::NineToTen,
            TimeSlot::TenToEleven,
            TimeSlot::ElevenToTwelve,
            TimeSlot::TwelveToThirteen,
            TimeSlot::ThirteenToFourteen,
            TimeSlot::FourteenToFifteen,
            TimeSlot::FifteenToSixteen,
            TimeSlot::SixteenToSeventeen,
            TimeSlot::SeventeenToEighteen,
            TimeSlot::EighteenToNineteen
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NineToTen => "09:00-10:00",
            Self::TenToEleven => "10:00-11:00",
            Self::ElevenToTwelve => "11:00-12:00",
            Self::TwelveToThirteen => "12:00-13:00",
            Self::ThirteenToFourteen => "13:00-14:00",
            Self::FourteenToFifteen => "14:00-15:00",
            Self::FifteenToSixteen => "15:00-16:00",
            Self::SixteenToSeventeen => "16:00-17:00",
            Self::SeventeenToEighteen => "17:00-18:00",
            Self::EighteenToNineteen => "18:00-19:00",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_slot_as_str() {
        assert_eq!(TimeSlot::NineToTen.as_str(), "09:00-10:00");
        assert_eq!(TimeSlot::EighteenToNineteen.as_str(), "18:00-19:00");
    }

    #[test]
    fn test_time_slot_all_slots() {
        let slots = TimeSlot::all_slots();
        assert_eq!(slots.len(), 10);
    }
}
