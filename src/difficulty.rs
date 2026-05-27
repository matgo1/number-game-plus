use std::io;

use crate::inout;

pub enum Difficulties {
    Easy,
    Medium,
    Hard,
}

fn process_response(input: &str) -> Difficulties {
    match input {
        "1" => Difficulties::Easy,
        "2" => Difficulties::Medium,
        "3" => Difficulties::Hard,
    }
}

pub fn set_difficulty() -> Result<(u32, u32), io::Error> {
    loop {
        inout::ask_difficulty();

        let user_dif = inout::get_user_input()?;
        if user_dif.trim() == "0" {
            inout::write_difficulties_manual();
            continue;
        } else {
            process_response(&user_dif);
        }
    }
}
