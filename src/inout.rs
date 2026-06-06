use std::io::{self, Write};

pub fn prompt(message: &str) -> io::Result<()> {
    print!("{message}");
    io::stdout().flush()?;
    Ok(())
}

pub fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(String::from(input.trim()))
}
