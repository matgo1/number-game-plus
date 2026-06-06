use crate::inout;
use crate::inout::prompt;
use crate::messages::*;

pub struct Config {
    answers_range: std::ops::RangeInclusive<u16>,
    tries_count: u8,
}

pub enum Difficulties {
    // Maybe add some more Difficulties
    Easy,   // Numbers 1-50, 7 tries
    Medium, // Numbers 1-100, 5 tries
    Hard,   // Numbers 1-500, 3 tries
}

impl Difficulties {
    pub fn create_settings(&self) -> Config {
        match self {
            Difficulties::Easy => Config {
                answers_range: 1..=50,
                tries_count: 7,
            },

            Difficulties::Medium => Config {
                answers_range: 1..=100,
                tries_count: 5,
            },

            Difficulties::Hard => Config {
                answers_range: 1..=500,
                tries_count: 3,
            },
        }
    }
}

pub fn select_difficulty() -> Result<Config, std::io::Error> {
    // Loop asking for correct choice of dif
    loop {
        inout::prompt(ASK_DIFFICULTY)?;
        let user_choice = inout::read_line()?;

        match user_choice.as_str() {
            "1" => break Ok(Difficulties::Easy.create_settings()),
            "2" => break Ok(Difficulties::Medium.create_settings()),
            "3" => break Ok(Difficulties::Hard.create_settings()),
            "?" => explain_diffs(),
            _ => prompt("Please enter valid option")?,
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
