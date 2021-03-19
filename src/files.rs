use std::fs::read_dir;
use std::io::{stdout, stdin, Write};
use crossterm::terminal::{Clear, ClearType};
use crossterm::execute;

pub fn find_audio_files() -> Vec<String> {
    print!("Which folder do you want me to look for songs in? Please write down the full path, and please do use backslashes '\\'\n> ");
    stdout().flush().unwrap();
    let mut directory = String::new();
    stdin()
        .read_line(&mut directory)
        .expect("Failed to read line.");

    let mut files = vec!();
    let contents = read_dir(directory.trim()).expect("That is an invalid directory.");
    for content in contents {
        let file_name =  content.unwrap().path().display().to_string();
        let length_of_file_name = file_name.chars().count();
        let extensions = file_name[length_of_file_name-4..].to_string();

        if extensions == ".mp3" || extensions == ".wav" {
            files.push(file_name);
        }
    }
    return files;
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
    execute!(stdout, Clear(ClearType::All)).unwrap();
    for (mut index, name) in file_name.iter().enumerate() {
        index += 1;
        println!("{}. {}", index, name);
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