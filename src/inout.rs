use std::io::{self, Write};

pub fn prompt(message: &str) -> io::Result<()> {
    // Function for writing messages
    print!("{message}");
    io::stdout().flush()?;
    Ok(())
}

pub fn read_line() -> io::Result<String> {
    //Function for reading input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(String::from(input.trim()))
}
