mod inout;
mod messages;
use messages::*;

use crate::inout::prompt;

fn main() -> Result<(), std::io::Error> {
    inout::prompt(WELCOME)?; // Welcome messages

    // Loop for checking user input
    loop {
        inout::prompt(MAIN_MENU)?;

        // Read user input
        let user_mode_choice = inout::read_line()?;

        // Analyze user input
        match user_mode_choice.trim().parse::<u8>() {
            Ok(choice) => match choice {
                1 => {
                    inout::prompt("option 1 is chosen")?;
                    break;
                }
                2 => {
                    inout::prompt("option 2 is chosen")?;
                    break;
                }
                3 => {
                    inout::prompt("option 3 is chosen")?;
                    break;
                }
                _ => {
                    prompt("Please, choice a valid number")?;
                }
            },
            Err(_) => {
                inout::prompt("Please, eneter a number")?;
            }
        }
    }

    Ok(())
}
