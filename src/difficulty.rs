use crate::inout;
use crate::inout::prompt;
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
        }
    }
}

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
    loop {
        inout::prompt(ASK_DIFFICULTY)?;
        let user_choice = inout::read_line()?;

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

fn explain_diffs() {
    loop {
        inout::prompt(EXPLAIN_DIFFICULTIES).unwrap();
        let user_choice = inout::read_line().unwrap();

        match user_choice.as_str() {
            "q" => break,
            _ => continue,
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
