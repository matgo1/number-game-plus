mod difficulty;
mod game;
mod inout;
mod messages;
mod random;
mod stats;
use messages::*;

fn main() -> Result<(), std::io::Error> {
    inout::prompt(WELCOME); // Welcome messages
    inout::prompt(MAIN_MENU); // Display main menu

    let user_input = inout::read_line()?;

    Ok(())
}
