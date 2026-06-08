use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Stats {
    pub played_games: u16,
    pub wins: u16,
    pub losses: u16,
}

pub fn load_stats() -> Stats {
    let filename = "data/stats.json";

    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {
            return Stats::default();
        }
        Err(err) => panic!("Could not open file: {err}"),
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        panic!("Could not read the file: {err}");
    }

    if contents.trim().is_empty() {
        return Stats::default();
    }

    let stats: Stats = match serde_json::from_str(&contents) {
        Ok(data) => data,
        Err(err) => panic!("Could not parse JSON: {err}"),
    };

    stats
}

pub fn save_stats(stats: &Stats) {
    let filename = "data/stats.json";

    if let Err(err) = fs::create_dir_all("data") {
        panic!("Could not create a directory: {err}");
    }

    let json_string = match serde_json::to_string_pretty(stats) {
        Ok(string) => string,
        Err(err) => panic!("Could not seriallize Stats to JSON: {err}"),
    };

    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(err) => panic!("Could not open/create the file for writin: {err}"),
    };

    if let Err(err) = file.write_all(json_string.as_bytes()) {
        panic!("Could not write to file: {err}");
    }
}
