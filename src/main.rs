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

fn playback(file_to_play: &String) {
    //! ### Summary
    //! Plays the select audio file via the provided file path
    //! 
    //! ### Detailed explanation
    //! It creates an audio source from the result of `File::open(file_to_play)`, appends it to the sink
    //! and plays the audio file. It can also detect keyboard inputs while audio is playing, this is used to
    //! control the volume, and to exit the player.
    //! 
    //! ### An example
    //! ```
    //! let path_to_audio_file = <path to audio file>.to_string();
    //! playback(&path_to_audio_file);
    //! ```
    let mut stdout = stdout();
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // opens the file, makes an audio source from the file
    let file = File::open(file_to_play).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file));
    let source = match source {
        Ok(source) => source,
        Err(_error) => {
            let error_style = style::Style::new([244, 25, 9]);
            style::stylized_output(&error_style, "The file you've chosen has an unrecognized format.".to_string());
            std::process::abort()
        }
    };

    // Create a new thread to play audio on.
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);

    loop {
        let volume = sink.volume();

        key::detect(&sink, &mut stdout, volume);
        if sink.empty() { break; }
    }
}

fn main() {
    let mut stdout = stdout();

    let file_path = files::find_audio_files();
    let file_path = match file_path {
        Some(file_path) => file_path,
        None => std::process::abort()
    };
    let file_name = files::get_song_names(&file_path);
    let song_to_play = files::select_song(&file_name, &file_path);

    let mut current_song = files::get_song_name(&song_to_play);
    let current_song_style = style::Style::new([135, 244, 9]);

    execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::FromCursorDown)).unwrap();
    current_song = format!("Currently playing: {}", current_song);
    style::stylized_output(&current_song_style, current_song.to_string());
    println!();

    playback(&song_to_play);

    loop {
        let file_name = files::get_song_names(&file_path);
        let song_to_play = files::select_song(&file_name, &file_path);

        current_song = files::get_song_name(&song_to_play);
        current_song = format!("Currently playing: {}", current_song);

        execute!(stdout, cursor::MoveTo(0, 0), Clear(ClearType::FromCursorDown)).unwrap();
        style::stylized_output(&current_song_style, current_song.to_string());
        println!();

        playback(&song_to_play);
    }
}