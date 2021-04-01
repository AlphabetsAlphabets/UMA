use std::fs;
use std::io::prelude::*;
use std::fs::OpenOptions;

use serde::{Serialize, Deserialize};
use serde_json;

use super::files;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub dir: String,
    pub first: u8
}

pub fn deserialize_json() -> Config {
    let content = fs::read_to_string("./src/confs/startup.json").expect("Unable to open JSON config file.");
    let json: Config = serde_json::from_str(&content).expect("Unable to parse JSON.");

    json
}

pub fn first_time_setup(json: &Config) -> bool {
    let is_first_time = json.first;
    if is_first_time == 1 {
        // Ask for dir
        return true;
    }
    else {
        return false;
    }
}

pub fn modify_json() -> Config {
    let mut json = deserialize_json();
    let is_first_time = first_time_setup(&json) ;

    if !is_first_time {
        return json;
    }

    let directory = files::find_audio_directory();
    json.dir = directory;
    json.first = 0;

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .open("src/confs/startup.json")
        .expect("Unable to open file with permissions.");

    let data = serde_json::to_string(&json).expect("Unable to serialize struct.");
    let data = data.as_bytes();
    file.write_all(data).expect("Unable to write to file.");

    return json;
}
