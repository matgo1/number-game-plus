use std::io;

// Print start menu
pub fn write_hello_message() {
    println!("===Random Number Game===\n");
    println!("Choose:");
    println!("1. Start game");
    println!("2. Show your stats");
    println!("3. Quit")
}

// Function for reading user input
pub fn get_user_input() -> Result<String, io::Error> {
    let mut user_choice = String::new();

    io::stdin().read_line(&mut user_choice)?;
    Ok(user_choice.trim().to_string())
}

// Ask user which difficulty he want
pub fn ask_difficulty() {
    println!("Choose difficulty");
    println!("1. Easy");
    println!("2. Medium");
    println!("3. Hard");
    println!("0. Info")
}

// TODO: Add some manual here
pub fn difficulty_expl() {
    println!("TODO!")
}
