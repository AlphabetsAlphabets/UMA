use std::clone::Clone;
use std::fs::{read_dir, OpenOptions};
use std::io::{stdin, stdout, Write};

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};

use super::config;
use super::style;

pub fn find_audio_directory() -> String {
    print!("Which folder do you want me to look for songs in?.\n> ");
    stdout().flush().expect("Unable to flush.");

    let mut directory = String::new();
    stdin()
        .read_line(&mut directory)
        .expect("Failed to read line.");

    let directory = directory.trim();

    // Creates a Path, converts it to a string, then converts &str to String
    let directory = Path::new(&directory)
        .to_str()
        .expect("Unable to convert to string.")
        .to_owned();

    directory
}

pub fn get_file_extension(directory: String) -> Option<Vec<PathBuf>> {
    let mut files = vec![];

    // Read the contents of the directory
    let contents = read_dir(&directory);
    let contents = match contents {
        Ok(directory) => directory,
        Err(_error) => {
            let error_style = style::Style(244, 25, 9);
            let error_message = format!("{} is not a valid directory.\n", directory);

            let help_style = style::Style(32, 252, 226);

            let dir = config::get_config_dir();
            let json = config::Config {
                first: 1,
                dir: "".to_string(),
            };

            let dir = format!("{}/startup.json", dir.to_owned().display());

            let data = serde_json::to_string(&json).expect("unable to serialize struct.");
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(dir)
                .expect("Unable to create file object with desired permissions.");

            file.write_all(data.as_bytes())
                .expect("Unable to write to file.");

            style::stylized_output(&error_style, &error_message);
            style::stylized_output(&help_style, "Help: Did you end the file path without a '/'?\n");

            return None;
        }
    };

    for content in contents {
        let file_name = content.expect("Unable to read dir.").path();
        let extension = file_name
            .extension()
            .expect("File does not have an extension.");

        if extension == "mp3" || extension == "wav" {
            files.push(file_name);
        }
    }

    return Some(files);
}

pub fn get_song_names(songs: &[PathBuf]) -> Vec<OsString> {
    let mut names = Vec::new();
    for song in songs {
        let song_name = song.file_name().expect("The file has no name.");

        names.push(song_name.to_owned());
    }

    return names;
}

pub fn select_song(file_name: &[OsString], file_path: &[PathBuf]) -> Vec<PathBuf> {
    let mut stdout = stdout();
    // Look into queues instead of execute
    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All))
        .expect("Unable to execute crossterm functions");

    for (mut index, name) in file_name.iter().enumerate() {
        let name = name.clone().into_string();
        let name = match name {
            Ok(name) => name,
            Err(_error) => panic!("Unable to retrieve string from OsString."),
        };

        index += 1;
        let choose_song = format!("{}. {}", index, name);
        let choose_song_style = style::Style(252, 242, 108);
        style::stylized_output(&choose_song_style, &choose_song);
        println!();
    }

    let mut response = String::new();
    stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    let mut song = response
        .trim()
        .parse::<usize>()
        .expect("Unable to convert to int");

    song -= 1;
    let song_name = (&file_path[song]).to_owned();

    let mut song_names: Vec<PathBuf> = vec![];
    song_names.push(song_name);

    song_names
}
