use directories_next::ProjectDirs;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json;

use super::files;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub dir: String,
    pub first: u8,
}

pub fn get_config_dir() -> PathBuf {
    let dir = ProjectDirs::from("UMA", "UMA", "uma").expect("Unable to create project dir.");
    let dir = dir.config_dir();

    return dir.to_owned();
}

pub fn deserialize_json() -> Config {
    let dir = get_config_dir();
    let dir = format!("{}/startup.json", dir.display());
    let content = fs::read_to_string(dir).expect("Unable to open JSON config file.");

    let json: Config = serde_json::from_str(&content).expect("Unable to parse JSON.");

    json
}

/// # Summary
/// Creates the directory .uma at home, then creates startup.json in the directory
pub fn create_config() {
    let dir = get_config_dir();

    let status = "Unable to create directory at (~/.config/uma)";
    fs::create_dir(&dir).expect(status);

    let config_json = format!("{}/startup.json", dir.display());

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_json)
        .expect("Unable to create file object with desired permissions.");

    let json = Config {
        dir: "".to_string(),
        first: 1,
    };
    let data = serde_json::to_string(&json).expect("unable to serialize struct.");
    file.write(data.as_bytes())
        .expect("Unable to write to file.");
}

pub fn first_time_setup(json: &Config) -> bool {
    let is_first_time = json.first;
    if is_first_time == 1 {
        return true;
    } else {
        return false;
    }
}

pub fn reset_json() {
    let dir = get_config_dir();
    let dir = format!("{}/startup.json", dir.display());
    let json = Config {
        first: 1,
        dir: "".to_string(),
    };

    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(dir)
        .expect("Unable to reset json file.");

    let json =
        serde_json::to_string_pretty(&json).expect("Unable to serialize struct. In reset_json()");

    file.write_all(json.as_bytes())
        .expect("Unable to reset json.");
}

pub fn modify_json() -> Config {
    let dir = get_config_dir();

    let does_config_exist = Path::new(&dir).exists();
    if !does_config_exist {
        create_config();
    }

    let mut json = deserialize_json();
    let is_first_time = first_time_setup(&json);

    if !is_first_time {
        return json;
    }

    let directory = files::find_audio_directory();
    json.dir = directory;
    json.first = 0;

    let dir = format!("{}/startup.json", dir.display());

    let mut file = OpenOptions::new()
        .write(true)
        .open(dir)
        .expect("Unable to open file with permissions.");

    let data = serde_json::to_string(&json).expect("Unable to serialize struct.");
    let data = data.as_bytes();
    file.write_all(data).expect("Unable to write to file.");

    return json;
}
