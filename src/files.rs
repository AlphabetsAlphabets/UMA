use std::fs::read_dir;
use std::io::{stdout, stdin, Write};
use std::clone::Clone;

use std::ffi::OsString;
use std::path::{PathBuf, Path};

use crossterm::{execute, cursor};
use crossterm::terminal::{Clear, ClearType};

use super::style;

pub fn find_audio_files() -> Option<Vec<PathBuf>> {
    print!("Which folder do you want me to look for songs in? Supports absolute and relative paths.\n> ");
    stdout().flush().expect("Unable to flush.");
    
    let mut directory = String::new();
    stdin()
        .read_line(&mut directory)
        .expect("Failed to read line.");

    let directory = directory.trim();

    let directory = Path::new(&directory);

    let mut files = vec![];
    let contents = read_dir(directory);
    let contents = match contents {
        Ok(directory) => directory,
        Err(_error) => {
            let error_style = style::Style::new([244, 25, 9]);
            let error_message = format!("{} is not a valid directory.", directory.display());
            style::stylized_output(&error_style, &error_message);
            return None;
        }
    };

    for content in contents {
        let file_name =  content.expect("Unable to read dir.").path();
        let extension = file_name.extension().expect("File does not have an extension.");

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
    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)).expect("Unable to execute crossterm functions");

    for (mut index, name) in file_name.iter().enumerate() {
        let name = name.clone().into_string();
        let name = match name {
            Ok(name) => name,
            Err(_error) => panic!("Unable to retrieve string from OsString.")
        };

        index += 1;
        let choose_song = format!("{}. {}", index, name);
        let choose_song_style = style::Style::new([252, 242, 108]);
        style::stylized_output(&choose_song_style, &choose_song);
        println!();
    }

    let mut response = String::new();
    stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    let mut song = response.trim().parse::<usize>().expect("Unable to convert to int");
    song -= 1;
    let song_name = (&file_path[song]).to_owned();

    let mut song_names: Vec<PathBuf> = vec![];
    song_names.push(song_name);

    return song_names;
}