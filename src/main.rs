use std::io;

mod difficulty;
mod game;
mod inout;
mod random;
mod stats;

fn main() -> Result<(), io::Error> {
    loop {
        inout::write_hello_message();
        let choice = inout::get_user_input()?;

        match choice.as_str() {
            "1" => {
                println!("Starting game...")
            }
            "2" => {
                println!("Writing stats...")
            }
            "3" => {
                break;
            }

            "67" => {
                println!("Fuck")
            }

            _ => {
                println!("Wrong input")
            }
        }
    }

    Ok(())
}
