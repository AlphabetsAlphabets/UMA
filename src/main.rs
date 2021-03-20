use std::io::stdout;
use std::fs::File;
use std::io::BufReader;

use rodio::Sink;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, cursor};

mod style;
mod key;
mod files;

/*
TODO:
    1. Add a pretty selection screen.
*/

fn playback(file_to_play: String) {
    let mut stdout = stdout();

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // opens the file, makes an audio source from the file
    let file = File::open(file_to_play).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    // Create a new thread to play audio on.
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    loop {
        let volume = sink.volume();

        key::detect(&sink, &mut stdout, volume);
        let finished = sink.empty();
        if finished {
            break;         
        }
    }
}

fn main() {
    let mut stdout = stdout();

    let file_path = files::find_audio_files();
    let file_path = match file_path {
        Some(file_path) => file_path,
        None => { 
            std::process::abort()
        }
    };
    loop {
        let mut _count = 0;
        if _count <= 0 {
            let file_name = files::get_song_names(&file_path);
            let song_to_play = files::select_song(&file_name, &file_path);

            let mut current_song = files::get_song_name(&song_to_play);
            let current_song_style = style::Style::new([135, 244, 9]);

            execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::FromCursorDown)).unwrap();
            current_song = format!("Currently playing: {}", current_song);
            style::stylized_output(&current_song_style, current_song.to_string());
            println!();
            
            playback(song_to_play);
            _count += 1;
        }
        else {
            let file_name = files::get_song_names(&file_path);
            let song_to_play = files::select_song(&file_name, &file_path);

            let mut current_song = files::get_song_name(&song_to_play);
            let current_song_style = style::Style::new([135, 244, 9]);

            execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::FromCursorDown)).unwrap();
            current_song = format!("Currently playing: {}", current_song);
            style::stylized_output(&current_song_style, current_song.to_string());
            println!();

            playback(song_to_play);
        }
    }
}