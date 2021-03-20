use std::fs::read_dir;
use std::io::{stdout, stdin, Write};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, cursor};

use super::style;

pub fn find_audio_files() -> Option<Vec<String>> {
    print!("Which folder do you want me to look for songs in? \
    Please write down either the full path or relative path, \
    and please do use backslashes '\\'\n> ");
    
    stdout().flush().unwrap();
    let mut directory = String::new();
    stdin()
        .read_line(&mut directory)
        .expect("Failed to read line.");

    let mut files = vec!();
    let contents = read_dir(directory.trim());
    let contents = match contents {
        Ok(directory) => directory,
        Err(_error) => {
            let error_style = style::Style::new([244, 25, 9]);
            let error_message = format!("{} is not a valid directory.", directory.trim());
            style::stylized_output(&error_style, error_message.to_string());
            return Option::None;
        }
    };
    for content in contents {
        let file_name =  content.unwrap().path().display().to_string();
        let length_of_file_name = file_name.chars().count();
        let extensions = file_name[length_of_file_name-4..].to_string();

        if extensions == ".mp3" || extensions == ".wav" {
            files.push(file_name);
        }
    }
    return Option::Some(files);
}

pub fn get_song_name(song_path: &String) -> String {
    let split = song_path.split('\\');
    let chars = split.collect::<Vec<&str>>();
    let length = chars.iter().count() - 1;
    let select_name = chars[length].to_string();

    return select_name;
}

pub fn get_song_names(songs: &Vec<String>) -> Vec<String> {
    let mut names = Vec::new();
    for song in songs {
        let split = song.split('\\');
        let characters = split.collect::<Vec<&str>>();
        let length = characters.iter().count() - 1;
        let select_name = characters[length].to_string();

        names.push(select_name);
    }

    return names;
}

pub fn select_song(file_name: &Vec<String>, file_path: &Vec<String>) -> String {
    let mut stdout = stdout();
    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::All)).unwrap();
    for (mut index, name) in file_name.iter().enumerate() {
        index += 1;
        let choose_song = format!("{}. {}", index, name).to_string();
        let choose_song_style = style::Style::new([252, 242, 108]);
        style::stylized_output(&choose_song_style, choose_song);
        println!();
    }
    let mut response = String::new();
    stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    let mut song = response.trim().parse::<usize>().expect("Unable to convert to int");
    song -= 1;
    let song = &file_path[song];
    return song.to_string();
}