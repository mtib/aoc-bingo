use chrono::{DateTime, Datelike, TimeZone, Utc};

use crate::service::aoc_utils::AocUtils;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct AocPuzzle {
    pub date: PuzzleDate,
    pub part: AocPart,
}

impl Ord for AocPuzzle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.date.cmp(&other.date) {
            std::cmp::Ordering::Equal => self.part.cmp(&other.part),
            ord => ord,
        }
    }
}

impl PartialOrd for AocPuzzle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum AocPart {
    One,
    Two,
}

impl Into<u32> for AocPart {
    fn into(self) -> u32 {
        match self {
            AocPart::One => 1,
            AocPart::Two => 2,
        }
    }
}

impl From<u32> for AocPart {
    fn from(value: u32) -> Self {
        match value {
            1 => AocPart::One,
            2 => AocPart::Two,
            _ => panic!("Invalid AoC part: {}", value),
        }
    }
}

#[derive(Debug, Clone, Eq, serde::Serialize, serde::Deserialize, Hash)]
pub struct PuzzleDate {
    pub year: u32,
    pub day: u32,
}

impl PuzzleDate {
    pub fn to_datetime(&self) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(self.year as i32, 12, self.day, 0, 0, 0)
            .unwrap()
    }
    pub fn to_aoc_puzzle_part(&self, part: AocPart) -> AocPuzzle {
        AocPuzzle {
            date: self.clone(),
            part,
        }
    }
    pub fn new(year: u32, day: u32) -> Self {
        if day < 1 || day > 25 {
            panic!("Invalid year or day for AoC puzzle.");
        }
        PuzzleDate { year, day }
    }
    pub fn latest_of_year(year: u32) -> Option<Self> {
        let now = Utc::now();
        Self::latest_of_year_by_date(year, &now)
    }
    pub fn latest_of_year_by_date(year: u32, date: &DateTime<Utc>) -> Option<Self> {
        let current_year = date.year() as u32;
        if current_year == year && date.month() == 12 {
            Some(PuzzleDate::new(
                year,
                date.day()
                    .min(AocUtils::get_calendar_size_of_year(year).unwrap()),
            ))
        } else {
            match AocUtils::get_calendar_size_of_year(year) {
                Ok(size) => Some(PuzzleDate { year, day: size }),
                Err(_) => None,
            }
        }
    }
}

impl PartialEq for PuzzleDate {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.day == other.day
    }
}

impl Into<DateTime<Utc>> for PuzzleDate {
    fn into(self) -> DateTime<Utc> {
        self.to_datetime()
    }
}

impl Ord for PuzzleDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.year.cmp(&other.year) {
            std::cmp::Ordering::Equal => self.day.cmp(&other.day),
            ord => ord,
        }
    }
}

impl PartialOrd for PuzzleDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
