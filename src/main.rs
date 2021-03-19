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
    1. Ask where to locate mp3 files.
        - test if it's limited to mp3 only
    2. Add a pretty selection screen.
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

    let intro = style::Style::new([252, 121, 7]);

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    style::stylized_output(&intro, "Welcome to the music player.".to_string());
    println!();

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
    loop {
        let file_path = files::find_audio_files();
        let file_name = files::get_song_names(&file_path);
        let song_to_play = files::select_song(&file_name, &file_path);
        playback(song_to_play);
    }
}