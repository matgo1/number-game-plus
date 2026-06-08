mod difficulty;
mod inout;
mod logic;
mod messages;
mod random;
mod stats;
use messages::*;

#[derive(Debug, PartialEq)]
pub enum MenuChoice {
    Play,
    Stats,
    Quit,
}

fn parse_menu_choice(input: &str) -> Option<MenuChoice> {
    match input.trim() {
        "1" => Some(MenuChoice::Play),
        "2" => Some(MenuChoice::Stats),
        "3" => Some(MenuChoice::Quit),
        _ => None,
    }
}

fn main() -> Result<(), std::io::Error> {
    clearscreen::clear().expect("Failed to clear screen");
    inout::prompt(WELCOME)?; // Welcome messages

    let mut current_stats = stats::load_stats(); // Get stats from the file (or zeroes)

    // Loop for checking user input
    loop {
        inout::prompt(MAIN_MENU)?;

        // Read user input
        let user_mode_choice = inout::read_line()?;

        // Analyze user input
        match parse_menu_choice(&user_mode_choice) {
            Some(MenuChoice::Play) => {
                clearscreen::clear().expect("Failed to clear the screen");
                current_stats.played_games += 1; // One more plyed game

                // + Win if win or + Loss with lost
                if logic::start_game()? {
                    current_stats.wins += 1;
                } else {
                    current_stats.losses += 1;
                }
                stats::save_stats(&current_stats);
            }
            Some(MenuChoice::Stats) => {
                // print stats
                inout::prompt(format!("Current stats: {:?}", current_stats).as_str())?;
            }
            Some(MenuChoice::Quit) => {
                inout::prompt("Quit selected")?;
                stats::save_stats(&current_stats);
                break;
            }
            None => inout::prompt("Please enter a valid option (1 - 3)")?,
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(parse_menu_choice("1"), Some(MenuChoice::Play));
        assert_eq!(parse_menu_choice("2"), Some(MenuChoice::Stats));
        assert_eq!(parse_menu_choice("3"), Some(MenuChoice::Quit));
    }
}
