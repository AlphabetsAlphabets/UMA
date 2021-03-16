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

fn playback() {
    let mut stdout = stdout();

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // opens the file, makes an audio source from the file
    let file = File::open("src/audio/audio.mp3").unwrap();
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

fn find_audio_files() {
    print!("Which folder do you want me to look for songs in? Please write down the full path\n> ");
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
        let extensions = file_name[length_of_file_name-3..].to_string();

        if extensions == "mp3" || extensions == ".wav" {
            files.push(file_name);
        }
    }
    for file in files {
        println!("{}", file);
    }
}

fn main() {
    find_audio_files();
    playback();
}