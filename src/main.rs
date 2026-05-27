use std::io;

mod difficulty;
mod game;
mod inout;
mod random;
mod stats;

fn main() -> Result<(), io::Error> {
    // Loop of etntry interface
    loop {
        inout::write_hello_message(); // Starting message with mode choice

        let choice = inout::get_user_input()?; // Read user input

        // Different functionality depending on user choice
        match choice.as_str() {
            "1" => {
                println!("Starting game...") // Here will be started main game loop
            }
            "2" => {
                println!("Writing stats...") // Reading and writing stats from json
            }
            "3" => {
                break; // Quiting the app
            }

            "67" => {
                println!("Fuck") // Voice of mankind
            }

            _ => {
                println!("Wrong input") // For fools
            }
        }
    }

    Ok(()) // Return that everithing OK if everithing is OK
}
