<<<<<<< Updated upstream
use std::io;

use crate::inout;

pub enum Difficulties {
    Easy,
    Medium,
    Hard,
}

// Implementation of Difficulties
impl Difficulties {
    pub fn get_range(&self) -> std::ops::RangeInclusive<u32> {
        // Get range depends on chosen difficulty
        // Returns Range
        match self {
            Difficulties::Easy => 1..=50,
            Difficulties::Medium => 1..=100,
            Difficulties::Hard => 1..=500,
=======
use crate::inout;
use crate::messages::*;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub answers_range: std::ops::RangeInclusive<u16>,
    pub tries_count: u8,
}

pub enum Difficulty {
    // Maybe add some more Difficulties
    Easy,   // Numbers 1-50, 7 tries
    Medium, // Numbers 1-100, 5 tries
    Hard,   // Numbers 1-500, 3 tries
}

impl Difficulty {
    pub fn create_settings(&self) -> Config {
        match self {
            Difficulty::Easy => Config {
                answers_range: 1..=50,
                tries_count: 7,
            },

            Difficulty::Medium => Config {
                answers_range: 1..=100,
                tries_count: 5,
            },

            Difficulty::Hard => Config {
                answers_range: 1..=500,
                tries_count: 3,
            },
>>>>>>> Stashed changes
        }
    }
}

<<<<<<< Updated upstream
pub fn select_difficulty() -> Result<std::ops::RangeInclusive<u32>, io::Error> {
=======
fn parse_difficulty(input: &str) -> Option<Difficulty> {
    match input.trim() {
        "1" => Some(Difficulty::Easy),
        "2" => Some(Difficulty::Medium),
        "3" => Some(Difficulty::Hard),
        _ => None,
    }
}

pub fn select_difficulty() -> Result<Config, std::io::Error> {
    // Loop asking for correct choice of dif
>>>>>>> Stashed changes
    loop {
        // Loop in case of errors
        inout::ask_difficulty();

<<<<<<< Updated upstream
        let choice = inout::get_user_input()?;
=======
        match user_choice.as_str() {
            "?" => explain_diffs(),

            _ => {
                if let Some(difficulty) = parse_difficulty(&user_choice) {
                    return Ok(difficulty.create_settings());
                }

                inout::prompt("Please enter a valid option.\n")?;
            }
        }
    }
}
>>>>>>> Stashed changes

        match choice.to_lowercase().as_str() {
            "1" | "easy" => return Ok(Difficulties::Easy.get_range()),
            "2" | "medium" => return Ok(Difficulties::Medium.get_range()),
            "3" | "hard" => return Ok(Difficulties::Hard.get_range()),
            _ => inout::difficulty_expl(),
        }
    }
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    // Difficlty -> Config

    #[test]
    fn test_configs() {
        assert_eq!(
            Difficulty::Easy.create_settings(),
            Config {
                answers_range: 1..=50,
                tries_count: 7,
            }
        );
        assert_eq!(
            Difficulty::Medium.create_settings(),
            Config {
                answers_range: 1..=100,
                tries_count: 5,
            }
        );

        assert_eq!(
            Difficulty::Hard.create_settings(),
            Config {
                answers_range: 1..=500,
                tries_count: 3,
            }
        );
    }

    // Input Parsing tests

    #[test]
    fn parse_difficulties() {
        assert!(matches!(parse_difficulty("1"), Some(Difficulty::Easy)));
        assert!(matches!(parse_difficulty("2"), Some(Difficulty::Medium)));
        assert!(matches!(parse_difficulty("3"), Some(Difficulty::Hard)));
    }

    #[test]
    fn rejects_invalid_input() {
        assert!(parse_difficulty("banana").is_none());
    }

    #[test]
    fn rejects_empty_input() {
        assert!(parse_difficulty("").is_none());
    }

    #[test]
    fn trims_whitespaces() {
        assert!(matches!(parse_difficulty(" 1 "), Some(Difficulty::Easy)));
    }
}
