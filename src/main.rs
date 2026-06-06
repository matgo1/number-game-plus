mod difficulty;
mod inout;
mod messages;
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
    inout::prompt(WELCOME)?; // Welcome messages

    // Loop for checking user input
    loop {
        inout::prompt(MAIN_MENU)?;

        // Read user input
        let user_mode_choice = inout::read_line()?;

        // Analyze user input
        match parse_menu_choice(&user_mode_choice) {
            Some(MenuChoice::Play) => {
                inout::prompt("Play selected")?;
            }
            Some(MenuChoice::Stats) => {
                inout::prompt("Stats selected")?;
            }
            Some(MenuChoice::Quit) => {
                inout::prompt("Quit selected")?;
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
