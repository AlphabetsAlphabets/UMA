use std::io::{stdout, stdin, Write, self};
use std::fs::{File, read_dir};
use std::io::BufReader;

use rodio::Sink;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, cursor};

mod style;
mod key;

/*
TODO:
    1. Ask where to locate mp3 files.
        - test if it's limited to mp3 only
    2. Add a pretty selection screen.
*/

fn playback(file_path: String) {
    let mut stdout = stdout();

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // opens the file, makes an audio source from the file
    let file = File::open(file_path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    // Create a new thread to play audio on.
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    let intro = style::Style::new([252, 121, 7]);

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    style::stylized_output(&intro, "Welcome to the music player.".to_string());
    println!();

    loop {
        let volume = sink.volume();

        key::detect(&sink, &mut stdout, volume);
   }
}

// Everything from below here is searching for files

fn find_audio_files() -> Vec<String> {
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

/*
TODO:
1. Ask which folder to look for songs in.
    - Display names of the songs. [Done]

2. Ask which song to play first.   
    - Display which songs are available. [Done]
    - Select with either numbers or with a selection bar. [Done]
3. Play the selected song. [Done]
*/

/*
TODO:
1. Be able to skip to the next song. Or rewind to the previous song.
*/

fn get_song_names(songs: &Vec<String>) -> Vec<String> {
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

fn select_song(file_name: &Vec<String>, file_path: &Vec<String>) -> String {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All));
    for (mut index, name) in file_name.iter().enumerate() {
        index += 1;
        println!("{}. {}", index, name);
    }
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    let mut song = response.trim().parse::<usize>().expect("Unable to convert to int");
    song -= 1;
    let song = &file_path[song];
    return song.to_string();
}

fn main() {
    let file_path = find_audio_files();
    let file_name = get_song_names(&file_path);
    let song = select_song(&file_name, &file_path);
    playback(song);
}