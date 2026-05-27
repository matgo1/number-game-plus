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
        }
    }
}

pub fn select_difficulty() -> Result<std::ops::RangeInclusive<u32>, io::Error> {
    loop {
        // Loop in case of errors
        inout::ask_difficulty();

        let choice = inout::get_user_input()?;

        match choice.to_lowercase().as_str() {
            "1" | "easy" => return Ok(Difficulties::Easy.get_range()),
            "2" | "medium" => return Ok(Difficulties::Medium.get_range()),
            "3" | "hard" => return Ok(Difficulties::Hard.get_range()),
            _ => inout::difficulty_expl(),
        }
    }
}
