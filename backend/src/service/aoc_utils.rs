use chrono::{Datelike, Utc};

use crate::model::aoc::{AocPart, AocPuzzle, PuzzleDate};

pub struct AocUtils;

impl AocUtils {
    pub fn earliest_puzzle() -> PuzzleDate {
        PuzzleDate { year: 2015, day: 1 }
    }
    pub fn latest_puzzle() -> PuzzleDate {
        let now = Utc::now();
        Self::latest_puzzle_by_date(&now)
    }
    fn latest_puzzle_by_date(date: &chrono::DateTime<Utc>) -> PuzzleDate {
        let year = if date.month() == 12 {
            date.year() as u32
        } else {
            (date.year() - 1) as u32
        };
        PuzzleDate::latest_of_year_by_date(year, date).unwrap()
    }
    pub fn get_calendar_size_of_year(year: u32) -> Result<u32, &'static str> {
        if year < AocUtils::earliest_puzzle().year {
            Err("Year must be 2015 or later.")
        } else if year < 2025 {
            Ok(25)
        } else {
            Ok(12)
        }
    }
    pub fn puzzle_days_for_years(years: &[u32]) -> Vec<PuzzleDate> {
        years
            .iter()
            .filter_map(|&year| PuzzleDate::latest_of_year(year))
            .flat_map(|latest_puzzle_date| {
                (1..=latest_puzzle_date.day)
                    .map(move |day| PuzzleDate::new(latest_puzzle_date.year, day))
            })
            .collect()
    }
    pub fn puzzles_for_years(years: &[u32]) -> Vec<AocPuzzle> {
        Self::puzzle_days_for_years(years)
            .into_iter()
            .map(|date| {
                vec![
                    date.to_aoc_puzzle_part(AocPart::One),
                    date.to_aoc_puzzle_part(AocPart::Two),
                ]
            })
            .flatten()
            .collect()
    }
    pub fn estimate_difficulty(puzzle: &AocPuzzle) -> u32 {
        let year_progression = (puzzle.date.day - 1) as f64
            / (Self::get_calendar_size_of_year(puzzle.date.year).unwrap() - 1) as f64;
        (year_progression * 5.0 + 1.0).floor() as u32
            + match puzzle.part {
                AocPart::One => 0,
                AocPart::Two => 2,
            }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_get_calendar_size_of_year() {
        assert_eq!(
            super::AocUtils::get_calendar_size_of_year(2015).unwrap(),
            25
        );
        assert_eq!(
            super::AocUtils::get_calendar_size_of_year(2023).unwrap(),
            25
        );
        assert_eq!(
            super::AocUtils::get_calendar_size_of_year(2025).unwrap(),
            12
        );
        assert_eq!(
            super::AocUtils::get_calendar_size_of_year(2030).unwrap(),
            12
        );
        assert!(super::AocUtils::get_calendar_size_of_year(2010).is_err());
    }

    #[test]
    fn test_puzzle_days_for_years() {
        let puzzles = super::AocUtils::puzzle_days_for_years(&[2015, 2024]);
        assert_eq!(puzzles.len(), 50);
        assert_eq!(puzzles.first().unwrap(), &super::PuzzleDate::new(2015, 1));
        assert_eq!(puzzles.last().unwrap(), &super::PuzzleDate::new(2024, 25));
    }

    #[test]
    fn test_puzzles_for_years() {
        let puzzles = super::AocUtils::puzzles_for_years(&[2015, 2024]);
        assert_eq!(puzzles.len(), 2 * 25 * 2);
        assert_eq!(
            puzzles.first().unwrap(),
            &super::AocPuzzle {
                date: super::PuzzleDate::new(2015, 1),
                part: super::AocPart::One
            }
        );
        assert_eq!(
            puzzles.last().unwrap(),
            &super::AocPuzzle {
                date: super::PuzzleDate::new(2024, 25),
                part: super::AocPart::Two
            }
        );
    }

    #[test]
    fn test_latest_puzzle_by_date() {
        // using chrono::Utc.with_ymd_and_hms()
        let date = Utc.with_ymd_and_hms(2023, 11, 15, 0, 0, 0).unwrap();
        let latest_puzzle = super::AocUtils::latest_puzzle_by_date(&date);
        assert_eq!(latest_puzzle, super::PuzzleDate::new(2022, 25));

        let date = Utc.with_ymd_and_hms(2023, 12, 10, 0, 0, 0).unwrap();
        let latest_puzzle = super::AocUtils::latest_puzzle_by_date(&date);
        assert_eq!(latest_puzzle, super::PuzzleDate::new(2023, 10));

        let date = Utc.with_ymd_and_hms(2025, 1, 5, 0, 0, 0).unwrap();
        let latest_puzzle = super::AocUtils::latest_puzzle_by_date(&date);
        assert_eq!(latest_puzzle, super::PuzzleDate::new(2024, 25));
    }
}
