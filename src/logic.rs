use crate::inout::prompt;
use crate::random::generate_random_number;
use crate::{difficulty, inout};
use clearscreen::clear as clean;
use std::cmp::Ordering;

pub fn start_game() -> Result<bool, std::io::Error> {
    let config = difficulty::select_difficulty()?;
    clean().expect("Failed to clear the screen");

    let min_num: u16 = *config.answers_range.start();
    let max_num: u16 = *config.answers_range.end();
    let secret_number = generate_random_number(&config.answers_range);

    let max_tries = config.tries_count;
    let mut tries: u8 = 0;

    loop {
        let cnt_tries = max_tries - tries;
        inout::prompt(format!("You have {cnt_tries} tries\n").as_str())?;
        inout::prompt(format!("Guess between {min_num} and {max_num}\n").as_str())?;
        inout::prompt("Your guess >>> ")?;
        let user_number = inout::read_line()?;

        let user_number: u16 = match user_number.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_number.cmp(&secret_number) {
            Ordering::Less => {
                clean().expect("Faied to clear the screen");
                inout::prompt("Too small!\n")?;
            }
            Ordering::Greater => {
                clean().expect("Faied to clear the screen");
                inout::prompt("Too Big!\n")?;
            }
            Ordering::Equal => {
                println!("You win!");
                return Ok(true);
            }
        }

        tries += 1;

        if tries == max_tries {
            prompt("You lost!")?;
            return Ok(false);
        }
    }
}
